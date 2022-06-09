import fs from 'fs';

const DB_FILE = 'db.json';

function read<T>(): T {
  if (!fs.existsSync(DB_FILE)) return null;
  return JSON.parse(fs.readFileSync(DB_FILE, 'utf-8'));
}

function write(data: unknown) {
  fs.writeFileSync(DB_FILE, JSON.stringify(data));
}

export default { read, write };
