
#[derive(Debug)]
enum XMLDocType {
    REPLAY,
}


pub struct XMLHeaderTag {
    r#type: String,
    exever: String,
    exebuild: String,
    title: String,
}

pub struct XMLPlayerModelTag {
    id: String,
}

pub struct XMLTimesTag {
    best: usize,
    respawns: isize,
    stuntscore: usize,
    validable: usize,
}

pub struct XMLCheckpointsTag {
    cur: String,
}

pub struct XMLDescTag {
    envir: String,
    mood: String,
    maptype: String,
    mapstyle: String,
    displaycost: usize,
    r#mod: String,
}

pub struct XMLMapTag {
    uid: String,
    name: String,
    author: String,
    authorzone: String,
}
