import junoModule from '../../juno-nodejs/src/juno-node';

async function main() {
	const juno = junoModule.default('../../juno.sock');

	await juno.initialize('test', '1.0.0', {
		'http2': '1.0.0'
	});

	await juno.declareFunction('onRequest', async (args: any) => {
		return {
			contentType: 'json',
			body: JSON.stringify({
				success: true,
				applicationId: 'kai-sdk',
				version: '1.0.0'
			})
		};
	});

	await juno.callFunction('http2.get', {
		path: '/',
		function: 'test.onRequest'
	});

	await juno.callFunction('http2.listen', {
		port: 8080
	});
}

main();
