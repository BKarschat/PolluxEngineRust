struct voxel {
    // 0000tttttttdddzzzzzzyyyyyyxxxxxx
    // t = texture bit (<= 70)
    // d = dircetion(0,1,...,6)
    // z = z achse
    // y = y achse
    // x = x achse
    triangle_1: i32,
    triangle_2: i32,
}

pub struct chunks {
    // position offset; Every Chunks gets the own position offset to render the voxels correctly
    offset: i32,
    voxelArray: [voxel; 32],
}

pub struct objectModel {
    // objects can be placed in different chunks
    //
}
