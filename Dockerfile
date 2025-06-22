FROM rust:latest
LABEL authors="marcin"

# Install fish shell and dependencies for plugins
RUN apt-get update && apt-get install -y fish curl git fzf grc

# Set up fish plugins and apply default config for tide plugin
ENV FISH_CONFIG_DIR /root/.config/fish
RUN mkdir -p $FISH_CONFIG_DIR/functions
RUN curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish > $FISH_CONFIG_DIR/functions/fisher.fish
RUN fish -c "fisher install IlanCosman/tide && fisher install jethrokuan/fzf && fisher install jorgebucaran/autopair.fish && fisher install oh-my-fish/plugin-grc"
RUN fish -c "tide configure --auto --style=Rainbow --prompt_colors='True color' --show_time='24-hour format' --rainbow_prompt_separators=Slanted --powerline_prompt_heads=Round --powerline_prompt_tails=Flat --powerline_prompt_style='Two lines, character and frame' --prompt_connection=Dotted --powerline_right_prompt_frame=No --prompt_connection_andor_frame_color=Dark --prompt_spacing=Sparse --icons='Many icons' --transient=Yes"

# Backup the installed and configured settings
RUN mkdir -p /opt/fish_config_default \
    && cp -r $FISH_CONFIG_DIR/* /opt/fish_config_default/

WORKDIR /app/

COPY . .

# Copy the entrypoint script and make it executable
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

# Create directories for persistent fish history and config
RUN mkdir -p /root/.local/share/fish /root/.config/fish

# Define volumes for fish shell history and config
VOLUME ["/root/.local/share/fish", "/root/.config/fish"]

ENTRYPOINT ["entrypoint.sh"]
CMD ["cargo", "watch", "--why", "-x", "build"]
