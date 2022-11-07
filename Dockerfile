FROM debian:11

COPY server_files /server_files
RUN chmod +x /server_files/BeamMP-Server-debian-11

RUN apt-get update -y
RUN apt-get install -y liblua5.3-dev curl





CMD ["/server_files/BeamMP-Server-debian-11"]