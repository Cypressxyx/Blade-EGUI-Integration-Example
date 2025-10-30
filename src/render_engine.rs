pub struct RenderEngine {
    blade_context: blade_graphics::Context,
    surface: blade_graphics::Surface,
    blade_egui_gui_painter: blade_egui::GuiPainter,
    command_encoder: blade_graphics::CommandEncoder,
}

impl RenderEngine {
    pub fn new(window: &winit::window::Window) -> Self {
        // Initilize the blade context
        let blade_context = unsafe {
            blade_graphics::Context::init(blade_graphics::ContextDesc {
                presentation: true,
                validation: true,
                timing: true,
                capture: true,
                overlay: false,
                device_id: 0, //let blade choose
            })
            .unwrap()
        };

        // define the surface configuration how it will be rendered on
        let surface_size = blade_graphics::Extent {
            width: window.inner_size().width,
            height: window.inner_size().height,
            depth: 1,
        };

        let surface_usage = blade_graphics::TextureUsage::TARGET;
        let surface_display_sync = blade_graphics::DisplaySync::Block;
        let surface_color_space = blade_graphics::ColorSpace::Srgb;
        let surface_configuration = blade_graphics::SurfaceConfig {
            size: surface_size,
            usage: surface_usage,
            display_sync: surface_display_sync,
            color_space: surface_color_space,
            transparent: true,
            allow_exclusive_full_screen: false,
        };

        // create surface blade will use
        let surface = blade_context
            .create_surface_configured(window, surface_configuration)
            .unwrap();

        // Create GUI painter
        let blade_egui_gui_painter = blade_egui::GuiPainter::new(surface.info(), &blade_context);

        let command_encoder_configuration = blade_graphics::CommandEncoderDesc {
            name: "hyogokuIde",
            buffer_count: 2,
        };

        let command_encoder = blade_context.create_command_encoder(command_encoder_configuration);

        Self {
            blade_context,
            surface,
            blade_egui_gui_painter,
            command_encoder,
        }
    }

    pub fn render(
        &mut self,
        primitives: &[egui::ClippedPrimitive],
        textures: &egui::TexturesDelta,
        screen_desc: &blade_egui::ScreenDescriptor,
    ) {
        // Start the queue
        self.command_encoder.start();
        let frame = self.surface.acquire_frame();
        self.command_encoder.init_texture(frame.texture());

        self.blade_egui_gui_painter.update_textures(
            &mut self.command_encoder,
            textures,
            &self.blade_context,
        );

        if let mut pass = self.command_encoder.render(
            "render-the shit",
            blade_graphics::RenderTargetSet {
                colors: &[blade_graphics::RenderTarget {
                    view: frame.texture_view(),
                    init_op: blade_graphics::InitOp::Load,
                    finish_op: blade_graphics::FinishOp::Store,
                }],
                depth_stencil: None,
            },
        ) {
            self.blade_egui_gui_painter.paint(
                &mut pass,
                primitives,
                screen_desc,
                &self.blade_context,
            );
        }
        self.command_encoder.present(frame);
        let sync_point = self.blade_context.submit(&mut self.command_encoder);
        self.blade_egui_gui_painter.after_submit(&sync_point);
    }
}
