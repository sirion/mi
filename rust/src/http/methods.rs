/// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
pub const GET: &'static str = "GET";
/// The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
pub const POST: &'static str = "POST";
/// The PUT method replaces all current representations of the target resource with the request payload.
pub const PUT: &'static str = "PUT";
/// The DELETE method deletes the specified resource.
pub const DELETE: &'static str = "DELETE";
/// The HEAD method asks for a response identical to that of a GET request, but without the response body.
pub const HEAD: &'static str = "HEAD";
/// The OPTIONS method is used to describe the communication options for the target resource.
pub const OPTIONS: &'static str = "OPTIONS";
/// The CONNECT method establishes a tunnel to the server identified by the target resource.
pub const CONNECT: &'static str = "CONNECT";
/// The TRACE method performs a message loop-back test along the path to the target resource.
pub const TRACE: &'static str = "TRACE";
/// The PATCH method is used to apply partial modifications to a resource.
pub const PATCH: &'static str = "PATCH";
