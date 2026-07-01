mod swapchain;

use std::{fs, sync::Arc};

use anyhow::{Ok, Result};
use ash::vk;
use winit::window::Window;

use crate::app::engine::{renderer::swapchain::Swapchain, rendering_context::RenderingContext};

pub struct Renderer {
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    swapchain: Swapchain,
    context: Arc<RenderingContext>,
}

const SHADERS_DIR: &str = "res/shaders/";

fn load_shader_module(context: &RenderingContext, path: &str) -> Result<ash::vk::ShaderModule> {
    let code = fs::read(format!("{}{}", SHADERS_DIR, path))?;
    context.create_shader_module(&code)
}

impl Renderer {
    pub fn new(context: Arc<RenderingContext>, window: Arc<Window>) -> Result<Self> {
        let mut swapchain = Swapchain::new(context.clone(), window.clone())?;
        swapchain.resize()?;

        let vertex_shader = load_shader_module(context.as_ref(), "vert.spv")?;
        let fragment_shader = load_shader_module(context.as_ref(), "frag.spv")?;

        unsafe {
            let pipeline_layout = context
                .device
                .create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default(), None)?;

            let pipeline = context.create_graphisc_pipeline(
                swapchain.extent,
                swapchain.format,
                vertex_shader,
                fragment_shader,
                pipeline_layout,
                Default::default(),
            )?;

            context.device.destroy_shader_module(vertex_shader, None);
            context.device.destroy_shader_module(fragment_shader, None);

            Ok(Self {
                context,
                swapchain,
                pipeline,
                pipeline_layout,
            })
        }
    }

    pub fn resize(&mut self) -> Result<()> {
        self.swapchain.resize()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.context.device.destroy_pipeline(self.pipeline, None);
            self.context
                .device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
