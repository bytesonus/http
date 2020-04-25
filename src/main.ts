#!/usr/bin/env node
import JunoModule from '../../juno-nodejs/src/juno-node';
import * as express from 'express';
import { createServer, Server } from 'http';
import HttpConfig, { MiddlewareType } from './http-config';
const metaData = require('./package.json');

let configs: HttpConfig[] = [];
let listeningServer: Server;

async function main() {
	const juno = JunoModule.default('../juno.sock');
	HttpConfig.junoModule = juno;

	await juno.initialize(metaData.name, metaData.version);

	await juno.declareFunction('clearConfig', () => {
		configs = [];
	});

	await juno.declareFunction('use', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.USE, args.path ?? "/", args.function));
	});

	await juno.declareFunction('connect', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.CONNECT, args.path ?? "/", args.function));
	});

	await juno.declareFunction('delete', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.DELETE, args.path ?? "/", args.function));
	});

	await juno.declareFunction('get', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.GET, args.path ?? "/", args.function));
	});

	await juno.declareFunction('head', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.HEAD, args.path ?? "/", args.function));
	});

	await juno.declareFunction('options', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.OPTIONS, args.path ?? "/", args.function));
	});

	await juno.declareFunction('patch', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.PATCH, args.path ?? "/", args.function));
	});

	await juno.declareFunction('post', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.POST, args.path ?? "/", args.function));
	});

	await juno.declareFunction('put', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.PUT, args.path ?? "/", args.function));
	});

	await juno.declareFunction('trace', (args: any) => {
		configs.push(new HttpConfig(MiddlewareType.TRACE, args.path ?? "/", args.function));
	});

	await juno.declareFunction('listen', async (args: any) => {
		if (listeningServer) {
			await new Promise(resolve => {
				listeningServer.close(resolve);
			});
		}

		const app = express();

		configs.forEach(config => {
			switch (config.middlewareType) {
				case MiddlewareType.USE:
					app.use(config.path, config.executeRoute);
					break;
				case MiddlewareType.CONNECT:
					app.connect(config.path, config.executeRoute);
					break;
				case MiddlewareType.DELETE:
					app.delete(config.path, config.executeRoute);
					break;
				case MiddlewareType.GET:
					app.get(config.path, config.executeRoute);
					break;
				case MiddlewareType.HEAD:
					app.head(config.path, config.executeRoute);
					break;
				case MiddlewareType.OPTIONS:
					app.options(config.path, config.executeRoute);
					break;
				case MiddlewareType.PATCH:
					app.patch(config.path, config.executeRoute);
					break;
				case MiddlewareType.POST:
					app.post(config.path, config.executeRoute);
					break;
				case MiddlewareType.PUT:
					app.put(config.path, config.executeRoute);
					break;
				case MiddlewareType.TRACE:
					app.trace(config.path, config.executeRoute);
					break;
			}
		});

		let server = createServer(app);
		await new Promise(resolve => {
			listeningServer = server.listen(
				args.port ?? 3000,
				args.bindAddr ?? "127.0.0.1",
				resolve
			);
		});
	});
}

main()