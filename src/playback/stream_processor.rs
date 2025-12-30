use super::filters_manager::FiltersManager;
use serde_json::Value;

// Placeholder for the resource
pub struct AudioResource {
    pub filters: FiltersManager,
    // stream: ...
}

pub fn create_audio_resource(
    _track_info: &Value, // Info object
    initial_filters: &Value,
) -> AudioResource {
    let mut filters_manager = FiltersManager::new();
    filters_manager.update(initial_filters);
    
    AudioResource {
        filters: filters_manager,
    }
}
