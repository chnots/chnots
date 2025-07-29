// components/EndpointSettings.tsx
import { useState } from "react";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/common/component/ui/table";
import { Card } from "@/common/component/ui/card";
import { Label } from "@/common/component/ui/label";
import { SyncEndpoint } from "../po";

export const EndpointSettings = () => {
  const [endpoints, setEndpoints] = useState<SyncEndpoint[]>([]);
  const [newEndpoint, setNewEndpoint] = useState<SyncEndpoint>({
    ip: "",
    port: -1,
  });

  const handleAddEndpoint = () => {
    if (!newEndpoint.ip || !newEndpoint.port) return;

    setEndpoints([
      ...endpoints,
      {
        ...newEndpoint,
      },
    ]);
    setNewEndpoint({ ip: "", port: -1 });
  };

  const handleDeleteEndpoint = (ip: string, port: number) => {
    setEndpoints(endpoints.filter((ep) => ep.ip !== ip || ep.port != port));
  };

  const handleSyncNow = (ip: string, port: number) => {
    setEndpoints(
      endpoints.map((ep) =>
        ep.ip === ip && ep.port === port
          ? { ...ep, lastSynced: new Date().toISOString().split("T")[0] }
          : ep
      )
    );
  };

  return (
    <Card className="p-6">
      <div className="space-y-6">
        <div className="flex items-end gap-4">
          <div className="flex-1 space-y-2">
            <Label htmlFor="endpoint-url">IP</Label>
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
            <Label htmlFor="endpoint-url">Port</Label>
            <Input
              id="endpoint-port"
              type="number"
              value={newEndpoint.port}
              onChange={(e) =>
                setNewEndpoint({
                  ...newEndpoint,
                  port: parseInt(e.target.value),
                })
              }
              placeholder="3011"
            />
          </div>

          <Button onClick={handleAddEndpoint}>Add Endpoint</Button>
        </div>

        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>IP</TableHead>
              <TableHead>Port</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {endpoints.map((endpoint) => (
              <TableRow key={endpoint.ip + endpoint.port}>
                <TableCell>{endpoint.ip}</TableCell>
                <TableCell>{endpoint.port}</TableCell>
                <TableCell>
                  <div className="flex gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleSyncNow(endpoint.ip, endpoint.port)}
                    >
                      Sync Now
                    </Button>
                    <Button
                      variant="destructive"
                      size="sm"
                      onClick={() =>
                        handleDeleteEndpoint(endpoint.ip, endpoint.port)
                      }
                    >
                      Delete
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </Card>
  );
};
