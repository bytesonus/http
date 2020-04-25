declare global {
	namespace Express {
		interface Request {
			data?: any;
		}
	}
}

export {};
