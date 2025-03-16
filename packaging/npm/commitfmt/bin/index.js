#!/usr/bin/env node
// @ts-check

var spawn = require("child_process").spawn;
const { getBinaryPath } = require("../get-bin");

var command_args = process.argv.slice(2);

var child = spawn(getBinaryPath(), command_args, { stdio: "inherit" });
child.on("close", process.exit);
