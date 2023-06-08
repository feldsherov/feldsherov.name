FROM alpine:3.18.0

RUN apk add --update \
    samba-common-tools \
    samba-client \
    samba-server \
    && rm -rf /var/cache/apk/*

ENV USER=fshare
ENV UID=25001
ENV GID=25001

RUN addgroup --gid ${GID} ${USER}

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "$(pwd)" \
    --ingroup "$USER" \
    --no-create-home \
    --uid "$UID" \
    "$USER"

ADD .samba_pw /.pw
RUN (cat /.pw ; cat /.pw) | passwd ${USER}
RUN (cat /.pw ; cat /.pw) | smbpasswd -a ${USER}
RUN rm .pw

EXPOSE 445/tcp

ENTRYPOINT ["smbd", "--foreground", "--no-process-group"]