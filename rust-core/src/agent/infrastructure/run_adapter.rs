use crate::{agent::domain::dto::RunDto, entities::run};

impl From<run::Model> for RunDto {
    fn from(model: run::Model) -> Self {
        RunDto {
            id: model.id.to_string(),
            created_at: model.created_at,
            assistant_id: model.assistant_id,
            thread_id: model.thread_id.map(|id| id.to_string()),
            instructions: model.instructions,
            model: model.model,
            status: model.status,
            metadata: model.metadata,
            temperature: model.temperature,
        }
    }
}

impl From<RunDto> for run::Model {
    fn from(dto: RunDto) -> Self {
        run::Model {
            id: dto.id.parse().unwrap(),
            created_at: dto.created_at,
            assistant_id: dto.assistant_id,
            thread_id: dto.thread_id.map(|id| id.parse().unwrap()),
            instructions: dto.instructions,
            model: dto.model,
            status: dto.status,
            metadata: dto.metadata,
            temperature: dto.temperature,
        }
    }
}
