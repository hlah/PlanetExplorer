use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::RenderDevice,
    },
};

pub const ATTRIBUTE_ALTITUDE: MeshVertexAttribute =
    MeshVertexAttribute::new("Altitude", 319173648, VertexFormat::Float32);

#[derive(Default, Debug, Clone, AsStd140, TypeUuid)]
#[uuid = "679eb5cb-7d9e-4c62-93b0-3abb2ba9c542"]
pub struct PlanetMaterial {
    pub max_altitude: f32,
    pub min_altitude: f32,
    pub color_mode: u32,
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for PlanetMaterial {
    type ExtractedAsset = Self;

    type PreparedAsset = GpuCustomMaterial;

    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extract_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: extract_asset.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            bind_group,
            _buffer: buffer,
        })
    }
}

impl Material for PlanetMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        return Some(asset_server.load("shaders/custom_shader.wgsl"));
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        return Some(asset_server.load("shaders/custom_shader.wgsl"));
    }

    fn bind_group(gpu_material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &gpu_material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(PlanetMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
        })
    }

    fn specialize(
        _: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            ATTRIBUTE_ALTITUDE.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
