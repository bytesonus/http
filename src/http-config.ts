import * as express from "express";
import JunoModule from "../../juno-nodejs/src/juno-node";

export default class HttpConfig {
	public static junoModule: JunoModule;

	public middlewareType: MiddlewareType;
	public path: string;
	public functionName: string;

	constructor(middlewareType: MiddlewareType, path: string, functionName: string) {
		this.middlewareType = middlewareType;
		this.path = path;
		this.functionName = functionName;
	}

	public async executeRoute(req: express.Request, res: express.Response, next: express.NextFunction) {
		const reqObject = fromRequest(req);
		const response: any = await HttpConfig.junoModule.callFunction(this.functionName, reqObject);
		if (response.next === true) {
			req.data = Object.assign(req.data, response.data);
			next();
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

			if (response.status) {
				res.status(response.status);
			}

			if (response.json) {
				res.json(response.json);
			} else {
				res.send(response.body);
			}
		}
	}
}

export interface ReqObject {
	httpVersionMajor: number;
	httpVersionMinor: number;
	httpVersion: string;
	headers: any;
	rawHeaders: string[];
	url: string;
	method: string;
	baseUrl: string;
	originalUrl: string;
	params: any;
	query: any;
	body: any;
}

function fromRequest(req: express.Request): ReqObject {
	return {
		httpVersionMajor: req.httpVersionMajor,
		httpVersionMinor: req.httpVersionMinor,
		httpVersion: req.httpVersion,
		headers: req.headers,
		rawHeaders: req.rawHeaders,
		url: req.url,
		method: req.method,
		baseUrl: req.baseUrl,
		originalUrl: req.originalUrl,
		params: req.params,
		query: req.query,
		body: req.body,
	};
}

export enum MiddlewareType {
	USE,
	CONNECT,
	DELETE,
	GET,
	HEAD,
	OPTIONS,
	PATCH,
	POST,
	PUT,
	TRACE,
}