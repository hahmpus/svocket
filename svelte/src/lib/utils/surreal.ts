import Surreal from 'surrealdb.js';

let token = '';
const db = new Surreal('http://127.0.0.1:8000/rpc');

try {
	await db.signin({
		user: 'root',
		pass: 'root',
	});
	await db.use({ns: 'test', db: 'test'});

} catch (e) {
	console.error('[surreal.ts]', e);
}

export default db;