use bytemuck::{Pod, Zeroable};
use game_of_life_engine::Cell;
use std::collections::HashSet;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Params {
    width_in_u32s: u32,
    height: u32,
    cols: u32,
    _padding: u32,
}

use rand::RngExt;

pub struct LifeEngine {
    cols: u32,
    rows: u32,
    width_in_u32s: u32,
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group_a: wgpu::BindGroup,
    bind_group_b: wgpu::BindGroup,
    buffer_a: wgpu::Buffer,
    buffer_b: wgpu::Buffer,
    is_a_current: bool,
}

impl LifeEngine {
    pub fn new(cols: u32, rows: u32) -> Self {
        pollster::block_on(Self::new_async(cols, rows))
    }

    async fn new_async(cols: u32, rows: u32) -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits {
                        max_storage_buffer_binding_size: 1536 * 1024 * 1024, // Support up to 1.5GB
                        max_buffer_size: 1536 * 1024 * 1024,
                        ..wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let width_in_u32s = (cols + 31) / 32;
        let buffer_size = (width_in_u32s * rows * 4) as wgpu::BufferAddress;

        let params = Params {
            width_in_u32s,
            height: rows,
            cols,
            _padding: 0,
        };

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Params Buffer"),
            contents: bytemuck::cast_slice(&[params]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let buffer_a = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer A"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_b = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer B"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group_a = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group A"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_a.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer_b.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let bind_group_b = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group B"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_b.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer_a.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            cols,
            rows,
            width_in_u32s,
            device,
            queue,
            compute_pipeline,
            bind_group_a,
            bind_group_b,
            buffer_a,
            buffer_b,
            is_a_current: true,
        }
    }

    pub fn with_initial_cells(cols: u32, rows: u32, initial_cells: HashSet<Cell>) -> Self {
        let mut engine = Self::new(cols, rows);
        let cells: Vec<Cell> = initial_cells.into_iter().collect();
        let _ = engine.activate_cells(&cells);
        engine
    }

    pub fn activate_cells(&mut self, cells: &[Cell]) -> Result<(), String> {
        let mut data = self.download_grid();
        
        for cell in cells {
            if cell.x < self.cols && cell.y < self.rows {
                let idx = (cell.y * self.width_in_u32s + (cell.x / 32)) as usize;
                let bit = cell.x % 32;
                data[idx] |= 1 << bit;
            } else {
                return Err(format!("Cell out of bounds: {}, {}", cell.x, cell.y));
            }
        }

        let current_buffer = if self.is_a_current {
            &self.buffer_a
        } else {
            &self.buffer_b
        };

        self.queue.write_buffer(current_buffer, 0, bytemuck::cast_slice(&data));
        Ok(())
    }

    pub fn activate_cell(&mut self, x: u32, y: u32) -> Result<(), String> {
        self.activate_cells(&[Cell::new(x, y)])
    }

    pub fn next(&mut self) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { 
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, if self.is_a_current { &self.bind_group_a } else { &self.bind_group_b }, &[]);
            cpass.dispatch_workgroups((self.width_in_u32s + 63) / 64, self.rows, 1);
        }
        self.queue.submit(Some(encoder.finish()));
        self.is_a_current = !self.is_a_current;
    }

    pub fn get_alive_cells(&self) -> impl Iterator<Item = Cell> {
        self.get_alive_cells_vec().into_iter()
    }

    fn get_alive_cells_vec(&self) -> Vec<Cell> {
        let data = self.download_grid();
        let mut alive_cells = Vec::new();
        for y in 0..self.rows {
            for ux in 0..self.width_in_u32s {
                let val = data[(y * self.width_in_u32s + ux) as usize];
                if val == 0 { continue; }
                for bit in 0..32 {
                    if (val >> bit) & 1 == 1 {
                        let x = ux * 32 + bit;
                        if x < self.cols {
                            alive_cells.push(Cell::new(x, y));
                        }
                    }
                }
            }
        }
        alive_cells
    }

    pub fn get_alive_cells_count(&self) -> usize {
        let data = self.download_grid();
        data.iter().map(|&val| val.count_ones() as usize).sum()
    }

    pub fn generate_random_square(&mut self, top_left: Cell, size: u32) {
        let mut rng = rand::rng();
        let mut cells = Vec::new();
        for y in top_left.y..std::cmp::min(top_left.y + size, self.rows) {
            for x in top_left.x..std::cmp::min(top_left.x + size, self.cols) {
                if rng.random_bool(0.5) {
                    cells.push(Cell::new(x, y));
                }
            }
        }
        let _ = self.activate_cells(&cells);
    }

    fn download_grid(&self) -> Vec<u32> {
        let current_buffer = if self.is_a_current {
            &self.buffer_a
        } else {
            &self.buffer_b
        };

        let buffer_size = (self.width_in_u32s * self.rows * 4) as wgpu::BufferAddress;
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(current_buffer, 0, &staging_buffer, 0, buffer_size);
        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(wgpu::Maintain::Wait);

        if let Ok(Ok(())) = receiver.recv() {
            let data = buffer_slice.get_mapped_range();
            let result = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            staging_buffer.unmap();
            result
        } else {
            panic!("failed to download grid from GPU");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glider() {
        let mut engine = LifeEngine::new(10, 10);
        let glider = vec![
            Cell::new(1, 0),
            Cell::new(2, 1),
            Cell::new(0, 2),
            Cell::new(1, 2),
            Cell::new(2, 2),
        ];
        engine.activate_cells(&glider).unwrap();
        
        for _ in 0..4 {
            engine.next();
        }
        
        let alive: std::collections::HashSet<Cell> = engine.get_alive_cells().collect();
        assert!(alive.contains(&Cell::new(2, 1)));
        assert!(alive.contains(&Cell::new(3, 2)));
        assert!(alive.contains(&Cell::new(1, 3)));
        assert!(alive.contains(&Cell::new(2, 3)));
        assert!(alive.contains(&Cell::new(3, 3)));
        assert_eq!(alive.len(), 5);
    }
}
