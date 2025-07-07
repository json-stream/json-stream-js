const assert = require('assert');
const { parseStream, JsJsonStreamParser } = require('..');

// simple parse test
const result = parseStream('{"key": "value", "age": 23}');
assert.deepStrictEqual(result, { key: 'value', age: 23 });

// streaming test
const parser = new JsJsonStreamParser();
parser.addChunk('{"count": ');
parser.addChunk('42}');
assert.deepStrictEqual(parser.getResult(), { count: 42 });

console.log('All tests passed');

