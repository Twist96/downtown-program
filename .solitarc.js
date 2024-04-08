const path = require('path');
const programDir = path.join(__dirname, '..', 'downtown-program/programs/downtown-program');
const idlDir = path.join(__dirname, 'target/idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
    idlGenerator: 'anchor',
    programName: 'downtown_program',
    programId: 'CgGCmVn7W9zjKjAqw3ypEQfEEiJGSM1u87AzyEC81m5b',
    idlDir,
    sdkDir,
    binaryInstallDir,
    programDir,
};

///Users/matthewchukwuemeka/RustroverProjects/downtown-program/programs/downtown-program/Cargo.toml