use tonic::{Request, Response, Status };
use super::service::Service;
use crate::db::{ establish_connection, create_user };
use crate::users::{
    CreateUserRequest, 
    CreateUserResponse,
};

pub fn create_user_operation(
    service: &Service,
    request: Request<CreateUserRequest>,
) -> Result<Response<CreateUserResponse>, Status> {
    println!("start Create User operation.");
    let conn = &mut establish_connection(&service.config);
    let req = request.get_ref();
    println!("Request payload: {:#?}", &req);

    let user = create_user(conn, &req.name, &req.bio);
    let reply = CreateUserResponse {
        id: user.id,
    };

    println!("end Create User operation.");
    Ok(Response::new(reply))
}