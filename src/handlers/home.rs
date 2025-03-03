use actix_web::{ get, HttpResponse, Responder };

// =========================================================================================

#[get("/")]
pub async fn home() -> impl Responder {
    let body =
        r#"
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta name="description" content="Imoost is an open-source, self-hosted image optimization service built for Next.js." />
        <meta name="author" content="Bryan Cellier, https://bryancellier.fr" />
        <title>Imoost</title>
        <script src="https://cdn.tailwindcss.com"></script>
      </head>
      <body>
        <main class="flex min-h-dvh items-center bg-[#FAFAFA] justify-center p-4">
          <div class="flex flex-col items-center gap-y-6">
            <div class="space-y-1">
              <h3 class="text-center text-4xl font-bold">Imoost</h3>
              <p class="text-center max-w-[64rem]">
                Imoost is an open-source, self-hosted image optimization service built for Next.js. It leverages imgproxy to dynamically transform, resize, and compress your images for faster load times and better performance—all while giving you full control over your image infrastructure.
              </p>
            </div>
            <a
              class="rounded bg-black px-4 py-2 text-white hover:bg-black/85 transition-colors duration-150"
              href="https://github.com/kyomawa/imoost"
              >Link to the repo</a
            >
          </div>
        </main>
      </body>
    </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(body)
}

// =========================================================================================
