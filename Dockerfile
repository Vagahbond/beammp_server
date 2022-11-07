FROM debian:11

# Separate exe and storage so that persistant volume claim works

    # --config=/path/to/ServerConfig.toml
    #                     Absolute or relative path to the
    #                     Server Config file, including the
    #                     filename. For paths and filenames with
    #                     spaces, put quotes around the path.
    # --working-directory=/path/to/folder
    #                     Sets the working directory of the Server.
    #                     All paths are considered relative to this,
    #                     including the path given in --config.

COPY server_files /server_files
RUN chmod +x /server_files/BeamMP-Server-debian-11

RUN apt-get update -y
RUN apt-get install -y liblua5.3-dev curl





CMD ["/server_files/BeamMP-Server-debian-11"]