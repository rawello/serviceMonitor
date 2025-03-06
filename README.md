# Server Monitor API

## Описание

API для мониторинга сервера и управления Docker-контейнерами. Позволяет получать информацию о состоянии системы (аптайм, загрузка CPU, использование памяти и диска) и управлять Docker-контейнерами (запуск, остановка, получение списка контейнеров).

### Базовый URL

- **Локальный сервер**: `http://localhost:8089`

---

## Маршруты

### Docker

#### 1. Получить список всех Docker-контейнеров

- **Метод**: `GET`
- **Путь**: `/docker/containers`
- **Описание**: Возвращает список всех Docker-контейнеров.
- **Ответ**:
  - **200 OK**:
    ```json
    [
      {
        "Id": "string",
        "Image": "string",
        "Status": "string",
        "Created": 1234567890
      }
    ]
    ```
  - **500 Internal Server Error**:
    ```json
    {
      "error": "string"
    }
    ```

#### 2. Запустить Docker-контейнер

- **Метод**: `POST`
- **Путь**: `/docker/containers/{containerId}/start`
- **Описание**: Запускает Docker-контейнер по его ID.
- **Параметры**:
  - `containerId` (path, required): ID контейнера.
- **Ответ**:
  - **200 OK**:
    ```json
    {
      "message": "string"
    }
    ```
  - **500 Internal Server Error**:
    ```json
    {
      "error": "string"
    }
    ```

#### 3. Остановить Docker-контейнер

- **Метод**: `POST`
- **Путь**: `/docker/containers/{containerId}/stop`
- **Описание**: Останавливает Docker-контейнер по его ID.
- **Параметры**:
  - `containerId` (path, required): ID контейнера.
- **Ответ**:
  - **200 OK**:
    ```json
    {
      "message": "string"
    }
    ```
  - **500 Internal Server Error**:
    ```json
    {
      "error": "string"
    }
    ```

#### 4. Получить список контейнеров с PostgreSQL

- **Метод**: `GET`
- **Путь**: `/docker/containers/postgres`
- **Описание**: Возвращает список всех Docker-контейнеров с PostgreSQL.
- **Ответ**:
  - **200 OK**:
    ```json
    [
      {
        "Id": "string",
        "Image": "string",
        "Status": "string",
        "Created": 1234567890
      }
    ]
    ```
  - **500 Internal Server Error**:
    ```json
    {
      "error": "string"
    }
    ```

---

### System

#### 1. Получить аптайм сервера

- **Метод**: `GET`
- **Путь**: `/system/uptime`
- **Описание**: Возвращает время работы сервера в секундах.
- **Ответ**:
  - **200 OK**:
    ```json
    {
      "uptime": 123456
    }
    ```

#### 2. Получить загрузку CPU

- **Метод**: `GET`
- **Путь**: `/system/cpu-load`
- **Описание**: Возвращает текущую загрузку CPU в процентах.
- **Ответ**:
  - **200 OK**:
    ```json
    {
      "cpu_load": 50.25
    }
    ```

#### 3. Получить использование памяти

- **Метод**: `GET`
- **Путь**: `/system/memory`
- **Описание**: Возвращает информацию об использовании памяти.
- **Ответ**:
  - **200 OK**:
    ```json
    {
      "used_memory": 4096,
      "total_memory": 8192
    }
    ```

#### 4. Получить использование диска

- **Метод**: `GET`
- **Путь**: `/system/disk-usage`
- **Описание**: Возвращает использование диска в процентах.
- **Ответ**:
  - **200 OK**:
    ```json
    {
      "disk_usage": 75.5
    }
    ```

---

## Схемы данных

### Container

```json
{
  "type": "object",
  "properties": {
    "Command": {
      "type": "string",
      "description": "Команда, используемая для запуска контейнера."
    },
    "Created": {
      "type": "integer",
      "description": "Время создания контейнера в формате Unix timestamp."
    },
    "HostConfig": {
      "type": "object",
      "properties": {
        "NetworkMode": {
          "type": "string",
          "description": "Режим сети, используемый контейнером."
        }
      },
      "description": "Конфигурация хоста контейнера."
    },
    "Id": {
      "type": "string",
      "description": "Уникальный идентификатор контейнера."
    },
    "Image": {
      "type": "string",
      "description": "Имя образа, на основе которого создан контейнер."
    },
    "ImageID": {
      "type": "string",
      "description": "Уникальный идентификатор образа."
    },
    "Labels": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      },
      "description": "Метки (labels), связанные с контейнером."
    },
    "Mounts": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "Destination": {
            "type": "string",
            "description": "Путь внутри контейнера, куда примонтирован том."
          },
          "Mode": {
            "type": "string",
            "description": "Режим монтирования (например, 'ro' или 'rw')."
          },
          "Propagation": {
            "type": "string",
            "description": "Тип распространения монтирования."
          },
          "RW": {
            "type": "boolean",
            "description": "Флаг, указывающий, доступен ли том для записи."
          },
          "Source": {
            "type": "string",
            "description": "Источник монтирования (путь на хосте или имя тома)."
          },
          "Type": {
            "type": "string",
            "description": "Тип монтирования ('bind', 'volume' и т.д.)."
          }
        }
      },
      "description": "Список томов, примонтированных к контейнеру."
    },
    "Names": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "Список имен контейнера."
    },
    "NetworkSettings": {
      "type": "object",
      "properties": {
        "Networks": {
          "type": "object",
          "additionalProperties": {
            "type": "object",
            "properties": {
              "Aliases": {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "nullable": true,
                "description": "Псевдонимы сети."
              },
              "DNSNames": {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "nullable": true,
                "description": "DNS-имена сети."
              },
              "DriverOpts": {
                "type": "object",
                "nullable": true,
                "description": "Опции драйвера сети."
              },
              "EndpointID": {
                "type": "string",
                "nullable": true,
                "description": "ID конечной точки сети."
              },
              "Gateway": {
                "type": "string",
                "description": "Шлюз сети."
              },
              "GlobalIPv6Address": {
                "type": "string",
                "description": "Глобальный IPv6-адрес."
              },
              "GlobalIPv6PrefixLen": {
                "type": "integer",
                "description": "Длина префикса глобального IPv6-адреса."
              },
              "IPAMConfig": {
                "type": "object",
                "nullable": true,
                "description": "Конфигурация IPAM."
              },
              "IPAddress": {
                "type": "string",
                "description": "IPv4-адрес контейнера в сети."
              },
              "IPPrefixLen": {
                "type": "integer",
                "description": "Длина префикса IPv4-адреса."
              },
              "IPv6Gateway": {
                "type": "string",
                "description": "IPv6-шлюз сети."
              },
              "Links": {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "nullable": true,
                "description": "Ссылки на другие контейнеры в сети."
              },
              "MacAddress": {
                "type": "string",
                "description": "MAC-адрес контейнера."
              },
              "NetworkID": {
                "type": "string",
                "nullable": true,
                "description": "ID сети."
              }
            }
          }
        }
      },
      "description": "Настройки сети контейнера."
    },
    "Ports": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "IP": {
            "type": "string",
            "description": "IP-адрес, связанный с портом."
          },
          "PrivatePort": {
            "type": "integer",
            "description": "Приватный порт контейнера."
          },
          "PublicPort": {
            "type": "integer",
            "description": "Публичный порт на хосте."
          },
          "Type": {
            "type": "string",
            "description": "Тип порта ('tcp', 'udp')."
          }
        }
      },
      "description": "Список портов, открытых в контейнере."
    },
    "State": {
      "type": "string",
      "description": "Текущее состояние контейнера ('running', 'stopped' и т.д.)."
    },
    "Status": {
      "type": "string",
      "description": "Статус контейнера (например, 'Up 2 hours')."
    }
  },
  "required": [
    "Command",
    "Created",
    "HostConfig",
    "Id",
    "Image",
    "ImageID",
    "Labels",
    "Mounts",
    "Names",
    "NetworkSettings",
    "Ports",
    "State",
    "Status"
  ],
  "description": "Полное описание Docker-контейнера."
}
```
