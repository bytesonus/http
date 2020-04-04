import GothamModule from '../../gotham-nodejs/src/gotham-node';

async function main() {
	const gotham = GothamModule.default('../../gotham.sock');

	await gotham.initialize('test', '1.0.0', {
		'http2': '1.0.0'
	});

	await gotham.declareFunction('onRequest', async (args: any) => {
		return {
			contentType: 'json',
			body: JSON.stringify({
				success: true,
				applicationId: 'kai-sdk',
				version: '1.0.0'
			})
		};
	});

	await gotham.callFunction('http2.get', {
		path: '/',
		function: 'test.onRequest'
	});

	await gotham.callFunction('http2.listen', {
		port: 8080
	});
}

main();
