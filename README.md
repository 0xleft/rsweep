# yet another super fast port scanner written in rust

## Usage
```bash
# single host
rsweep --host 172.0.0.1 --verbose --threads 1000

# class a
rsweep --range 10.0.0.0-10.255.255.255 --verbose --threads 1000

# class b
rsweep --range 172.16.0.0-172.31.255.255 --verbose --threads 1000

# class c
rsweep --range 192.168.0.0-192.168.255.255 --verbose --threads 1000 --ports 22,80
```

```bash
```