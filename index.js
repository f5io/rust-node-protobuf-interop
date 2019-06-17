const protobuf = require('protobufjs');
const fetch = require('isomorphic-fetch');
const { join } = require('path');

const proto = protobuf.loadSync(join(__dirname, 'src/messages.proto'));
const Input = proto.lookupType('paybase.pip.getPolicy.Input');
const Output = proto.lookupType('paybase.pip.getPolicy.Output');

const input = Input.fromObject({ id: 'foo' });

fetch('http://127.0.0.1:8081', {
  method: 'POST',
  body: Input.encode(input).finish(),
  headers: { 'Content-Type': 'application/protobuf' },
})
  .then(result => result.buffer())
  .then(buffer => Output.decode(buffer))
  .then(console.log)
  .catch(console.error);

