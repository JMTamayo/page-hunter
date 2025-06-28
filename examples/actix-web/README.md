# USING ACTIX WEB WITH PAGE HUNTER
Use [actix-web](https://docs.rs/actix-web/4.6.0/actix_web/) to build a web server using `page-hunter` to paginate APIs responses.

This service uses the [API Colombia](https://api-colombia.com) project to obtain data and implement page-hunter models and functions. Review the project documentation at the following links:
- **Repository:** [https://github.com/Mteheran/api-colombia](https://github.com/Mteheran/api-colombia?tab=readme-ov-file)
- **Web page:** [https://api-colombia.com](https://api-colombia.com)
- **API doc:** [https://api-colombia.com/swagger/index.html](https://api-colombia.com/swagger/index.html)

To try this example on your local computer, you just need to locate to the respective folder and run the following command:

```bash
	cargo run --release
```

When the service is running, you can explore the documentation as follows:
- **Swagger UI:** http://localhost:8080/swagger-ui/
- **Rapidoc:** http://localhost:8080/rapidoc
- **Redoc:** http://localhost:8080/redoc
- **Scalar:**  http://localhost:8080/scalar

Enjoy it! 😀