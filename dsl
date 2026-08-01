#
# Ephemeria Sync Engine
# .ess = Ephemeria Sync Script
#

# Global settings
policy "mods" strict;
policy "config" ignore;

# File system description
dir "mods" {
    file "forge-api.jar" {
        url "https://api.server.com/forge.jar";
        hash "a3f5b7...";
    }
    file "optifine.jar" {
        url "https://api.server.com/optifine.jar";
        hash "e2c8a1...";
    }
}

dir "config" {
    file "server_settings.json" {
        url "https://api.server.com/settings.json";
        hash "9d4e2b...";
    }
}
