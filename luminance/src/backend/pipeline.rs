//! Pipeline backend interface.
//!
//! This interface defines the low-level API pipelines must implement to be usable.

use crate::backend::buffer::Buffer;
use crate::backend::framebuffer::Framebuffer as FramebufferBackend;
use crate::backend::shading_gate::ShadingGate as ShadingGateBackend;
use crate::backend::texture::{Texture, TextureBase};
use crate::pipeline::{PipelineError, PipelineState};
use crate::pixel::Pixel;
use crate::texture::Dimensionable;

pub unsafe trait PipelineBase: ShadingGateBackend + TextureBase {
  type PipelineRepr;

  unsafe fn new_pipeline(&mut self) -> Result<Self::PipelineRepr, PipelineError>;
}

pub unsafe trait Pipeline<D>: PipelineBase + FramebufferBackend<D>
where
  D: Dimensionable,
{
  unsafe fn start_pipeline(
    &mut self,
    framebuffer: &Self::FramebufferRepr,
    pipeline_state: &PipelineState,
  );
}

pub unsafe trait PipelineBuffer<T>: PipelineBase + Buffer<T>
where
  T: Copy,
{
  type BoundBufferRepr;

  unsafe fn bind_buffer(
    pipeline: &Self::PipelineRepr,
    buffer: &Self::BufferRepr,
  ) -> Result<Self::BoundBufferRepr, PipelineError>;

  unsafe fn buffer_binding(bound: &Self::BoundBufferRepr) -> u32;
}

pub unsafe trait PipelineTexture<D, P>: PipelineBase + Texture<D, P>
where
  D: Dimensionable,
  P: Pixel,
{
  type BoundTextureRepr;

  unsafe fn bind_texture(
    pipeline: &Self::PipelineRepr,
    texture: &Self::TextureRepr,
  ) -> Result<Self::BoundTextureRepr, PipelineError>
  where
    D: Dimensionable,
    P: Pixel;

  unsafe fn texture_binding(bound: &Self::BoundTextureRepr) -> u32;
}
