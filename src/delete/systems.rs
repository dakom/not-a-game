use crate::prelude::*;

use super::data::MarkForDeletion;

// deletes entities that are marked for deletion
// this occurs in the physics step
pub fn delete_sys(mut all_storages: AllStoragesViewMut) {
    let to_delete = all_storages.run(|deletes: View<MarkForDeletion>| {
        deletes
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
    });

    for id in to_delete {
        // must first remove it from the scenegraph hierarchy
        all_storages.run(|mut sg_storages: SceneGraphStoragesMut| {
            (
                &mut sg_storages.entities,
                &mut sg_storages.parents,
                &mut sg_storages.children,
            )
                .remove(id);
        });
        // then delete the entity entirely
        all_storages.delete_entity(id);
    }
}
