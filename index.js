'use strict';

const fs = require('fs');

class RustConfigParser {
  constructor() {
    this.parser = require('./index.node');
    this.configContent = fs.readFileSync('config.set').toString();
    this.regexContent = fs.readFileSync('regex.json').toString();
    this.structureContent = fs.readFileSync('structure.json').toString();
    this.format = 'JSON';
  }
  parse() {
    return this.parser.parse(
      this.configContent,
      this.regexContent,
      this.structureContent,
      this.format
    );
  }
}
let parser = new RustConfigParser();
console.log(parser.parse());
