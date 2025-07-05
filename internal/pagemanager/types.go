package pagemanager

type SSHConnection struct {
	Nickname string `toml:"nickname"`
	User     string `toml:"user"`
	Host     string `toml:"host"`
	Port     int    `toml:"port"`
}
