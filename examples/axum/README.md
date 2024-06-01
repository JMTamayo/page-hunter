# USING AXUM WITH PAGE HUNTER AND POSTGRESQL
Use [axum](https://docs.rs/crate/axum/0.7.5) to build a web server using `page-hunter` to paginate APIs responses and PostgreSQL to manage data.

This service uses a PostgreSQL database as a docker container to manage example data.

To try this example on your local computer, you just need to locate to the respective folder and run the following command:

1. Install sqlx-cli:
```bash
	make install-sqlx-cli
```

2. Run database container:
```bash
	make run-db-container
```

3. Run the application:
```bash
	make run
```

When the service is running, you can explore the documentation as follows:
- **Swagger UI:** http://localhost:8080/swagger-ui/

Enjoy it! 😀