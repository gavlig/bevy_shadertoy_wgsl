use bevy::{
    core_pipeline::node::MAIN_PASS_DEPENDENCIES,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        // render_resource::*,
        render_resource::{std140::AsStd140, *},
        renderer::{RenderContext, RenderDevice, RenderQueue},
        RenderApp,
        RenderStage,
    },
    window::WindowResized,
};

use std::borrow::Cow;

// mod bufferA;
// use bufferA::*;

mod textureA;
use textureA::*;

mod textureB;
use textureB::*;

mod textureC;
use textureC::*;

mod textureD;
use textureD::{extract_texture_d, queue_bind_group_d, TextureD, TextureDNode, TextureDPipeline};

pub const SIZE: (u32, u32) = (1280, 720);
pub const WORKGROUP_SIZE: u32 = 8;
pub const NUM_PARTICLES: u32 = 256;

const COMMON: &'static str = include_str!("common.wgsl");

const IMAGE_SHADER: &'static str = include_str!("image.wgsl");
pub const IMAGE_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    bevy::render::render_resource::Shader::TYPE_UUID,
    192598017680025719,
);

const TEXTURE_A_SHADER: &'static str = include_str!("texture_a.wgsl");
pub const TEXTURE_A_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    bevy::render::render_resource::Shader::TYPE_UUID,
    986988749367675188,
);

const TEXTURE_B_SHADER: &'static str = include_str!("texture_b.wgsl");
pub const TEXTURE_B_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    bevy::render::render_resource::Shader::TYPE_UUID,
    808999425257967014,
);

const TEXTURE_C_SHADER: &'static str = include_str!("texture_c.wgsl");
pub const TEXTURE_C_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    bevy::render::render_resource::Shader::TYPE_UUID,
    819348234244712380,
);

const TEXTURE_D_SHADER: &'static str = include_str!("texture_d.wgsl");
pub const TEXTURE_D_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    bevy::render::render_resource::Shader::TYPE_UUID,
    193535259211504032,
);

fn main() {
    // // not sure this works on wasm
    // let mut wgpu_options = WgpuLimits::default();
    // wgpu_options.max_bind_groups = 5;
    // wgpu_options.max_storage_buffers_per_shader_stage = 5;
    // wgpu_options.max_storage_textures_per_shader_stage = 5;
    // wgpu_options.max_inter_stage_shader_components = 5;

    let mut app = App::new();
    // app.insert_resource(wgpu_options)
    app.insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(WindowDescriptor {
        //     // uncomment for unthrottled FPS
        //     // vsync: false,
        //     ..default()
        // })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(ShadertoyPlugin)
        .add_startup_system(setup)
        .add_system(update_common_uniform)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image);

    commands.insert_resource(MainImage(image.clone()));

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.insert_resource(CommonUniform::default());

    //
    //
    //
    // Texture A: equivalent of Buffer A in Shadertoy
    let mut texture_a = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // &[255, 255, 255, 255],
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    texture_a.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_a = images.add(texture_a);

    commands.insert_resource(TextureA(texture_a));

    //
    //
    //
    // Texture B: equivalent of Buffer B in Shadertoy
    let mut texture_b = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // &[255, 255, 255, 255],
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    texture_b.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_b = images.add(texture_b);

    commands.insert_resource(TextureB(texture_b));

    //
    //
    //
    // Texture C: equivalent of Buffer C in Shadertoy
    let mut texture_c = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // &[255, 255, 255, 255],
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    texture_c.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_c = images.add(texture_c);

    commands.insert_resource(TextureC(texture_c));

    //
    //
    //
    // Texture D: equivalent of Buffer D in Shadertoy
    let mut texture_d = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // &[255, 255, 255, 255],
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    texture_d.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let texture_d = images.add(texture_d);

    commands.insert_resource(TextureD(texture_d));
}

#[derive(Component, Default, Clone, AsStd140)]
pub struct CommonUniform {
    pub iTime: f32,
    pub iTimeDelta: f32,
    pub iFrame: i32,
    pub iSampleRate: i32,

    pub iChannelTime: Vec4,
    pub iChannelResolution: Vec4,
    pub iDate: [i32; 4],

    pub iResolution: Vec2,
    pub iMouse: Vec2,
}

pub struct CommonUniformMeta {
    // buffer: UniformVec<CommonUniform>,
    buffer: Buffer,
    // bind_group: Option<BindGroup>,
}

fn update_common_uniform(
    mut common_uniform: ResMut<CommonUniform>,
    mut window_resize_event: EventReader<WindowResized>,
    time: Res<Time>,
) {
    // update resolution
    for window_resize in window_resize_event.iter() {
        common_uniform.iResolution.x = window_resize.width;
        common_uniform.iResolution.y = window_resize.height;
    }
    // update time
    common_uniform.iTime = time.seconds_since_startup() as f32;
    common_uniform.iTimeDelta = time.delta_seconds() as f32;

    // println!("{:?}", common_uniform.iTime);
}

pub struct ShadertoyPlugin;

pub struct ShaderHandles {
    pub image_shader: Handle<Shader>,
    pub texture_a_shader: Handle<Shader>,
    pub texture_b_shader: Handle<Shader>,
    pub texture_c_shader: Handle<Shader>,
    pub texture_d_shader: Handle<Shader>,
}

impl Plugin for ShadertoyPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        let render_device = render_app.world.resource::<RenderDevice>();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("common uniform buffer"),
            size: CommonUniform::std140_size_static() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        render_app
            .insert_resource(CommonUniformMeta {
                buffer: buffer.clone(),
                // bind_group: None,
            })
            // .insert_resource(CommonUniformMetaA {
            //     buffer: buffer.clone(),
            // })
            // .add_system_to_stage(RenderStage::Prepare, prepare_common_uniform)
            .add_system_to_stage(RenderStage::Prepare, prepare_common_uniform)
            // .add_system_to_stage(RenderStage::Prepare, prepare_common_uniform_a)
            .init_resource::<MainImagePipeline>()
            .add_system_to_stage(RenderStage::Extract, extract_main_image)
            .add_system_to_stage(RenderStage::Queue, queue_bind_group)
            .init_resource::<TextureAPipeline>()
            .add_system_to_stage(RenderStage::Extract, extract_texture_a)
            .add_system_to_stage(RenderStage::Queue, queue_bind_group_a)
            .init_resource::<TextureBPipeline>()
            .add_system_to_stage(RenderStage::Extract, extract_texture_b)
            .add_system_to_stage(RenderStage::Queue, queue_bind_group_b);
        // .init_resource::<TextureCPipeline>()
        // .add_system_to_stage(RenderStage::Extract, extract_texture_c)
        // .add_system_to_stage(RenderStage::Queue, queue_bind_group_c)
        // .init_resource::<TextureDPipeline>()
        // .add_system_to_stage(RenderStage::Extract, extract_texture_d)
        // .add_system_to_stage(RenderStage::Queue, queue_bind_group_d);

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node("main_image", MainNode::default());
        render_graph.add_node("texture_a", TextureANode::default());
        render_graph.add_node("texture_b", TextureBNode::default());
        // render_graph.add_node("texture_c", TextureCNode::default());
        // // render_graph.add_node("texture_d", TextureDNode::default());

        render_graph
            .add_node_edge("texture_a", "texture_b")
            .unwrap();

        // render_graph
        //     .add_node_edge("texture_b", "texture_c")
        //     .unwrap();

        // // render_graph
        // //     .add_node_edge("texture_c", "texture_d")
        // //     .unwrap();

        // // render_graph
        // //     .add_node_edge("texture_d", "main_image")
        // //     .unwrap();

        render_graph
            .add_node_edge("texture_b", "main_image")
            .unwrap();

        render_graph
            .add_node_edge("main_image", MAIN_PASS_DEPENDENCIES)
            .unwrap();
    }
}

pub struct MainImagePipeline {
    main_image_group_layout: BindGroupLayout,
}

impl FromWorld for MainImagePipeline {
    fn from_world(world: &mut World) -> Self {
        let main_image_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("main_layout"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    CommonUniform::std140_size_static() as u64,
                                ),
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::ReadWrite,
                                format: TextureFormat::Rgba8Unorm,
                                view_dimension: TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 2,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::ReadWrite,
                                format: TextureFormat::Rgba8Unorm,
                                view_dimension: TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 3,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::ReadWrite,
                                format: TextureFormat::Rgba8Unorm,
                                view_dimension: TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        // BindGroupLayoutEntry {
                        //     binding: 4,
                        //     visibility: ShaderStages::COMPUTE,
                        //     ty: BindingType::StorageTexture {
                        //         access: StorageTextureAccess::ReadWrite,
                        //         format: TextureFormat::Rgba8Unorm,
                        //         view_dimension: TextureViewDimension::D2,
                        //     },
                        //     count: None,
                        // },
                    ],
                });

        MainImagePipeline {
            main_image_group_layout,
        }
    }
}

#[derive(Deref)]
struct MainImage(Handle<Image>);

struct MainImageBindGroup {
    main_image_bind_group: BindGroup,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

fn import_shader(
    shader_skeleton: &str,
    shader_handle_untyped: HandleUntyped,
    shaders: &mut Assets<Shader>,
) -> Handle<Shader> {
    //
    // insert common code in every shader
    let image_source = shader_skeleton.replace("{{COMMON}}", &COMMON);

    let image_shader = Shader::from_wgsl(Cow::from(image_source));
    shaders.set_untracked(shader_handle_untyped.clone(), image_shader.clone());
    shader_handle_untyped.typed()
}

// write the extracted common uniform into the corresponding uniform buffer
pub fn prepare_common_uniform(
    common_uniform_meta: ResMut<CommonUniformMeta>,
    render_queue: Res<RenderQueue>,
    common_uniform: Res<CommonUniform>,
) {
    use bevy::render::render_resource::std140::Std140;
    let std140_common_uniform = common_uniform.as_std140();
    let bytes = std140_common_uniform.as_bytes();

    render_queue.write_buffer(
        &common_uniform_meta.buffer,
        0,
        bevy::core::cast_slice(&bytes),
    );
}

fn extract_main_image(
    mut commands: Commands,
    image: Res<MainImage>,
    mut shaders: ResMut<Assets<Shader>>,
    common_uniform: Res<CommonUniform>,
    // asset_server: Res<AssetServer>,
    // common_uniform: Res<CommonUniform>,
) {
    // insert common uniform only once
    commands.insert_resource(common_uniform.clone());

    commands.insert_resource(MainImage(image.clone()));

    let image_shader_handle = import_shader(IMAGE_SHADER, IMAGE_SHADER_HANDLE, &mut shaders);

    // let image_shader = Shader::from_wgsl(Cow::from(IMAGE_SHADER));
    // shaders.set_untracked(IMAGE_SHADER_HANDLE.clone(), image_shader);
    // let image_handle: Handle<Shader> = IMAGE_SHADER_HANDLE.clone().typed();

    let texture_a_shader_handle =
        import_shader(TEXTURE_A_SHADER, TEXTURE_A_SHADER_HANDLE, &mut shaders);

    let texture_b_shader_handle =
        import_shader(TEXTURE_B_SHADER, TEXTURE_B_SHADER_HANDLE, &mut shaders);

    let texture_c_shader_handle =
        import_shader(TEXTURE_C_SHADER, TEXTURE_C_SHADER_HANDLE, &mut shaders);

    let texture_d_shader_handle =
        import_shader(TEXTURE_D_SHADER, TEXTURE_D_SHADER_HANDLE, &mut shaders);

    // let main_load = asset_server.load("shaders/image_load.wgsl");

    let all_shader_handles = ShaderHandles {
        // image_shader: image_handle,
        image_shader: image_shader_handle,
        // image_shader: main_load,
        texture_a_shader: texture_a_shader_handle,
        texture_b_shader: texture_b_shader_handle,
        texture_c_shader: texture_c_shader_handle,
        texture_d_shader: texture_d_shader_handle,
    };

    commands.insert_resource(all_shader_handles);
}

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<MainImagePipeline>,

    gpu_images: Res<RenderAssets<Image>>,
    main_image: Res<MainImage>,
    texture_a_image: Res<TextureA>,
    texture_b_image: Res<TextureB>,
    // texture_c_image: Res<TextureC>,
    // texture_d_image: Res<TextureD>,
    render_device: Res<RenderDevice>,
    mut pipeline_cache: ResMut<PipelineCache>,
    all_shader_handles: Res<ShaderHandles>,
    common_uniform_meta: ResMut<CommonUniformMeta>,
) {
    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: Some(vec![pipeline.main_image_group_layout.clone()]),
        shader: all_shader_handles.image_shader.clone(),
        shader_defs: vec![],
        entry_point: Cow::from("init"),
    });

    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: None,
        layout: Some(vec![pipeline.main_image_group_layout.clone()]),
        shader: all_shader_handles.image_shader.clone(),
        shader_defs: vec![],
        entry_point: Cow::from("update"),
    });

    let main_view = &gpu_images[&main_image.0];
    let texture_a_view = &gpu_images[&texture_a_image.0];
    let texture_b_view = &gpu_images[&texture_b_image.0];
    // let texture_c_view = &gpu_images[&texture_c_image.0];

    let main_image_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("main_bind_group"),
        layout: &pipeline.main_image_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: common_uniform_meta.buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&texture_a_view.texture_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&texture_b_view.texture_view),
            },
            // BindGroupEntry {
            //     binding: 3,
            //     resource: BindingResource::TextureView(&texture_c_view.texture_view),
            // },
            BindGroupEntry {
                binding: 3,
                resource: BindingResource::TextureView(&main_view.texture_view),
            },
        ],
    });

    commands.insert_resource(MainImageBindGroup {
        main_image_bind_group,
        init_pipeline: init_pipeline.clone(),
        update_pipeline: update_pipeline.clone(),
    });
}

pub enum ShadertoyState {
    Loading,
    Init,
    Update,
}

pub struct MainNode {
    pub state: ShadertoyState,
}

impl Default for MainNode {
    fn default() -> Self {
        Self {
            state: ShadertoyState::Loading,
        }
    }
}

impl render_graph::Node for MainNode {
    fn update(&mut self, world: &mut World) {
        let pipeline_cache = world.resource::<PipelineCache>();

        let bind_group = world.resource::<MainImageBindGroup>();

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        match self.state {
            ShadertoyState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(init_pipeline_cache)
                {
                    self.state = ShadertoyState::Init
                }
            }
            ShadertoyState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(update_pipeline_cache)
                {
                    self.state = ShadertoyState::Update
                }
            }
            ShadertoyState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_group = world.resource::<MainImageBindGroup>();

        let init_pipeline_cache = bind_group.init_pipeline;
        let update_pipeline_cache = bind_group.update_pipeline;

        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor {
                label: Some("main_compute_pass"),
            });

        pass.set_bind_group(0, &bind_group.main_image_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            ShadertoyState::Loading => {}

            ShadertoyState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(init_pipeline_cache)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }

            ShadertoyState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(update_pipeline_cache)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}
