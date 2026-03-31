/** Network interface descriptor returned by backend discovery. */
export interface NetworkInterface {
	name: string;
	description: string | null;
	addresses: InterfaceAddress[];
	flags: InterfaceFlags;
}

export interface InterfaceAddress {
	addr: string;
	netmask: string | null;
	broadcast: string | null;
}

export interface InterfaceFlags {
	is_up: boolean;
	is_loopback: boolean;
	is_running: boolean;
}
