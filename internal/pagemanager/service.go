package pagemanager

import (
	"bytes"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"cahier/internal/core"

	"github.com/BurntSushi/toml"
	"github.com/vistormu/go-dsa/errors"
)

func NewSSHConnection(nickname, uri string) (SSHConnection, error) {
	nickname = sanitizeNickname(nickname)

	var user, host string
	var port int

	userAndRest := strings.Split(uri, "@")
	if len(userAndRest) != 2 {
		return SSHConnection{}, errors.New(HostStringParsing).With("issue", "uri must be in the format 'user@host:port'")
	}

	user = userAndRest[0]
	if user == "" {
		return SSHConnection{}, errors.New(HostStringParsing).With("issue", "host name cannot be empty")
	}

	hostAndPort := strings.Split(userAndRest[1], ":")
	if len(hostAndPort) == 1 {
		port = 22
	}

	host = hostAndPort[0]
	if host == "" {
		return SSHConnection{}, errors.New(HostStringParsing).With("issue", "host cannot be empty")
	}

	if len(hostAndPort) == 2 {
		var err error
		port, err = strconv.Atoi(hostAndPort[1])
		if err != nil {
			return SSHConnection{}, errors.New(HostStringParsing).With("issue", "port must be a valid integer")
		}
	} else {
		return SSHConnection{}, errors.New(HostStringParsing).With("issue", "port must be specified in the format 'user@host:port'")
	}

	return SSHConnection{
		Nickname: nickname,
		User:     user,
		Host:     host,
		Port:     port,
	}, nil
}

func SaveSSHConnectionToFile(connection SSHConnection) error {
	var buffer bytes.Buffer
	encoder := toml.NewEncoder(&buffer)
	err := encoder.Encode(connection)
	if err != nil {
		return errors.New(FileWriteError).With("issue", "failed to encode SSH connection to TOML format").Wrap(err)
	}

	filePath := filepath.Join(core.CahierDirectory, connection.Nickname+".toml")

	file, err := os.Create(filePath)
	if err != nil {
		return errors.New(FileWriteError).Wrap(err)
	}
	defer file.Close()

	_, err = file.Write(buffer.Bytes())
	if err != nil {
		return errors.New(FileWriteError).With("issue", "failed to write SSH connection to file").Wrap(err)
	}

	return nil
}

func LoadSSHConnectionFromFile(nickname string) (SSHConnection, error) {
	filePath := filepath.Join(core.CahierDirectory, nickname+".toml")

	file, err := os.Open(filePath)
	if err != nil {
		return SSHConnection{}, errors.New(FileWriteError).With("issue", "failed to open file").Wrap(err)
	}
	defer file.Close()

	var connection SSHConnection
	_, err = toml.DecodeFile(filePath, &connection)
	if err != nil {
		return SSHConnection{}, errors.New(FileWriteError).With("issue", "failed to decode TOML file").Wrap(err)
	}

	return connection, nil
}

func LoadAllSSHConnections() ([]SSHConnection, error) {
	files, err := os.ReadDir(core.CahierDirectory)
	if err != nil {
		return nil, errors.New(FileReadError).Wrap(err)
	}

	var connections []SSHConnection
	for _, file := range files {
		if !file.IsDir() && strings.HasSuffix(file.Name(), ".toml") {
			nickname := strings.TrimSuffix(file.Name(), ".toml")
			connection, err := LoadSSHConnectionFromFile(nickname)
			if err != nil {
				return nil, errors.New(FileReadError).With("issue", "failed to load SSH connection from file").Wrap(err)
			}
			connections = append(connections, connection)
		}
	}

	return connections, nil
}

func EditSSHConnectionFromFile(nickname string, updatedConnection SSHConnection) error {
	connection, err := LoadSSHConnectionFromFile(nickname)
	if err != nil {
		return errors.New(FileWriteError).With("issue", "failed to load existing SSH connection").Wrap(err)
	}

	connection.Nickname = updatedConnection.Nickname
	connection.User = updatedConnection.User
	connection.Host = updatedConnection.Host
	connection.Port = updatedConnection.Port

	return SaveSSHConnectionToFile(connection)
}

func SSHConnectionExists(nickname string) bool {
	filePath := filepath.Join(core.CahierDirectory, nickname+".toml")
	_, err := os.Stat(filePath)
	return !os.IsNotExist(err)
}

func RemoveSSHConnectionFromFile(nickname string) error {
	filePath := filepath.Join(core.CahierDirectory, nickname+".toml")
	err := os.Remove(filePath)
	if err != nil {
		return errors.New(FileWriteError).With("issue", "failed to remove SSH connection file").Wrap(err)
	}
	return nil
}

func ClearSSHConnections() error {
	connections, err := LoadAllSSHConnections()
	if err != nil {
		return err
	}

	for _, connection := range connections {
		err := RemoveSSHConnectionFromFile(connection.Nickname)
		if err != nil {
			return err
		}
	}

	return nil
}
