use runique::prelude::*;

pub async fn inject_auth(request: &mut Request) {
    let user = is_authenticated(&request.session).await;
    context_update!(request => {
        "user" => user,
    });
}
