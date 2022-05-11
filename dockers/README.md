# Dockerfiles

There are three `Dockerfile`s to build docker image for the application.
The first image is based on alpine, the image size was 19MB.
The second image uses distroless by Google, the image size was 24.2MB.
The final image did not successfully build for single binary executable.

By using the `Dockerfile`, copy and paste the content, and replace `btmeister` to your application name.

## References

* [軽量Dockerイメージに安易にAlpineを使うのはやめたほうがいいという話](https://blog.inductor.me/entry/alpine-not-recommended) (blog.inductor.me)
* [Base Image Journey 2018](https://speakerdeck.com/stormcat24/base-image-journey-2018) (speakerdeck.com)
