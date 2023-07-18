use utils::direction;
use utils::file;

pub fn no_no_ea(bb: u64) -> u64 {
    bb << (direction::NOEA + direction::NORT) & !(file::FILES[file::File::A as usize])
}

pub fn no_ea_ea(bb: u64) -> u64 {
    bb << (direction::NOEA + direction::EAST)
        & !(file::FILES[file::File::A as usize] | file::FILES[file::File::B as usize])
}

pub fn so_ea_ea(bb: u64) -> u64 {
    bb >> -(direction::SOEA + direction::EAST)
        & !(file::FILES[file::File::A as usize] | file::FILES[file::File::B as usize])
}

pub fn so_so_ea(bb: u64) -> u64 {
    bb >> -(direction::SOEA + direction::SOUT) & !(file::FILES[file::File::A as usize])
}

pub fn no_no_we(bb: u64) -> u64 {
    bb << (direction::NOWE + direction::NORT) & !(file::FILES[file::File::H as usize])
}

pub fn no_we_we(bb: u64) -> u64 {
    bb << (direction::NOWE + direction::WEST)
        & !(file::FILES[file::File::H as usize] | file::FILES[file::File::G as usize])
}

pub fn so_we_we(bb: u64) -> u64 {
    bb >> -(direction::SOWE + direction::WEST)
        & !(file::FILES[file::File::H as usize] | file::FILES[file::File::G as usize])
}

pub fn so_so_we(bb: u64) -> u64 {
    bb >> -(direction::SOWE + direction::SOUT) & !(file::FILES[file::File::H as usize])
}
