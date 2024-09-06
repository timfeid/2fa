export const load = (ctx) => {
	return {
		accessToken: ctx.locals.accessToken,
		user: ctx.locals.user
	};
};
