export async function POST(req) {
	return new Response(req.locals.accessToken, { status: 200 });
}
