#!/usr/bin/env node
import GothamModule from '../../gotham-nodejs/src/gotham-node';
import * as express from 'express';
import * as http from 'http';
const metaData = require('./package.json');

const app = express();
app.use(express.json());
app.use(express.urlencoded());

async function main() {
	const gotham = GothamModule.default('../gotham.sock');

	await gotham.initialize(metaData.name, metaData.version);

	await gotham.declareFunction('use', (args: any) => {
		app.use(args.path, async (req, res, next) => {
			const response: any = await gotham.callFunction(args.function, getReqObject(req));
			if (response.next === true) {
				next(response.data);
			} else {
				if (response.contentType) {
					res.contentType(response.contentType);
				}
				if (response.headers) {
					for (const header in response.headers) {
						if (response.headers.hasOwnProperty(header)) {
							res.setHeader(header, response.headers[header]);
						}
					}
				}
				if (typeof(response.body) !== typeof('') || typeof(response.body) !== typeof(0)) {
					res.send(JSON.stringify(response.body));
				} else {
					res.send(response.body);
				}
			}
		});
	});

	await gotham.declareFunction('get', async (args: any) => {
		app.get(args.path, async (req, res, next) => {
			const response: any = await gotham.callFunction(args.function, getReqObject(req));
			if (response.next === true) {
				next(response.data);
			} else {
				if (response.contentType) {
					res.contentType(response.contentType);
				}
				if (response.headers) {
					for (const header in response.headers) {
						if (response.headers.hasOwnProperty(header)) {
							res.setHeader(header, response.headers[header]);
						}
					}
				}
				if (typeof(response.body) !== typeof('') || typeof(response.body) !== typeof(0)) {
					res.send(JSON.stringify(response.body));
				} else {
					res.send(response.body);
				}
			}
		});
	});

	await gotham.declareFunction('post', (args: any) => {
		app.post(args.path, async (req, res, next) => {
			const response: any = await gotham.callFunction(args.function, getReqObject(req));
			if (response.next === true) {
				next(response.data);
			} else {
				if (response.contentType) {
					res.contentType(response.contentType);
				}
				if (response.headers) {
					for (const header in response.headers) {
						if (response.headers.hasOwnProperty(header)) {
							res.setHeader(header, response.headers[header]);
						}
					}
				}
				if (typeof(response.body) !== typeof('') || typeof(response.body) !== typeof(0)) {
					res.send(JSON.stringify(response.body));
				} else {
					res.send(response.body);
				}
			}
		});
	});

	await gotham.declareFunction('patch', (args: any) => {
		app.patch(args.path, async (req, res, next) => {
			const response: any = await gotham.callFunction(args.function, getReqObject(req));
			if (response.next === true) {
				next(response.data);
			} else {
				if (response.contentType) {
					res.contentType(response.contentType);
				}
				if (response.headers) {
					for (const header in response.headers) {
						if (response.headers.hasOwnProperty(header)) {
							res.setHeader(header, response.headers[header]);
						}
					}
				}
				if (typeof(response.body) !== typeof('') || typeof(response.body) !== typeof(0)) {
					res.send(JSON.st