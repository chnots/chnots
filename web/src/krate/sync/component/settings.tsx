import { Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import { Label } from "@/common/component/ui/label";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/common/component/ui/table";
import type { SyncEndpoint } from "../po";
import {
  getSyncAllEndpoints,
  overwriteSyncAllEndpoints,
  syncToEndpoint,
} from "../service";

export const EndpointSettings = () => {
  const [endpoints, setEndpoints] = useState<SyncEndpoint[]>([]);
  const [newEndpoint, setNewEndpoint] = useState<SyncEndpoint>({
    ip: "",
    port: 3011,
  });
  useEffect(() => {
    getSyncAllEndpoints({}).then((rsp) => {
      setEndpoints(rsp.data.endpoints);
    });
  }, []);

  const handleAddEndpoint = async () => {
    if (!newEndpoint.ip || !newEndpoint.port) return;
    const eps = [
      ...endpoints,
      {
        ...newEndpoint,
      },
    ];
    await overwriteSyncAllEndpoints({
      data: {
        endpoints: eps,
      },
    });
    const backendEps = await getSyncAllEndpoints({});
    setEndpoints(backendEps.data.endpoints);
    setNewEndpoint({ ip: "", port: 3011 });
  };

  const handleDeleteEndpoint = async (ip: string, port: number) => {
    const eps = endpoints.filter((ep) => ep.ip !== ip || ep.port !== port);
    await overwriteSyncAllEndpoints({
      data: {
        endpoints: eps,
      },
    });
    const backendEps = await getSyncAllEndpoints({});
    setEndpoints(backendEps.data.endpoints);
  };

  const handleSyncNow = (ip: string, port: number) => {
    const eps = endpoints.filter((ep) => ep.ip === ip && ep.port === port);
    eps.forEach((e) => {
      syncToEndpoint({ endpoint: e });
    });
  };

  return (
    <div className="container mx-auto py-8">
      <div className="flex justify-between items-center mb-6">
        <form className="flex items-end gap-4">
          <div className="flex-1 space-y-2">
            <Label htmlFor="endpoint-ip">IP</Label>
            <Input
              id="endpoint-ip"
              value={newEndpoint.ip}
              onChange={(e) =>
                setNewEndpoint({ ...newEndpoint, ip: e.target.value })
              }
              placeholder="127.0.0.1"
            />
          </div>
          <div className="flex-1 space-y-2">
            <Label htmlFor="endpoint-port">Port</Label>
            <Input
              id="endpoint-port"
              type="number"
              value={newEndpoint.port}
              onChange={(e) =>
                setNewEndpoint({
                  ...newEndpoint,
                  port: parseInt(e.target.value, 10),
                })
              }
              placeholder="3011"
            />
          </div>

          <Button
            type="button"
            onClick={handleAddEndpoint}
            disabled={!newEndpoint.ip || !newEndpoint.port}
          >
            Add Endpoint
          </Button>
        </form>
      </div>

      <div className="border rounded-lg">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>IP</TableHead>
              <TableHead>Port</TableHead>
              <TableHead className="text-right">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {endpoints.length > 0 ? (
              endpoints.map((endpoint) => (
                <TableRow key={endpoint.ip + endpoint.port}>
                  <TableCell>{endpoint.ip}</TableCell>
                  <TableCell>{endpoint.port}</TableCell>
                  <TableCell className="text-right">
                    <div className="flex justify-end gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() =>
                          handleSyncNow(endpoint.ip, endpoint.port)
                        }
                      >
                        Sync Now
                      </Button>
                      <Button
                        variant="destructive"
                        size="icon"
                        onClick={() =>
                          handleDeleteEndpoint(endpoint.ip, endpoint.port)
                        }
                        aria-label={`Delete endpoint ${endpoint.ip}:${endpoint.port}`}
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell colSpan={3} className="h-24 text-center">
                  No endpoints found
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
};
