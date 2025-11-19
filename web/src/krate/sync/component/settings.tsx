// components/EndpointSettings.tsx
import { useEffect, useState } from 'react';

import { getSyncAllEndpoints, overwriteSyncAllEndpoints, syncToEndpoint } from '../service';

import type { SyncEndpoint } from '../po';
import { Button } from '@/common/component/ui/button';
import { Card } from '@/common/component/ui/card';
import { Input } from '@/common/component/ui/input';
import { Label } from '@/common/component/ui/label';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/common/component/ui/table';

export const EndpointSettings = () => {
  const [endpoints, setEndpoints] = useState<SyncEndpoint[]>([]);
  const [newEndpoint, setNewEndpoint] = useState<SyncEndpoint>({
    ip: '',
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
    setNewEndpoint({ ip: '', port: 3011 });
  };

  const handleDeleteEndpoint = async (ip: string, port: number) => {
    const eps = endpoints.filter((ep) => ep.ip !== ip || ep.port != port);
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
    <Card className="p-6">
      <div className="space-y-6">
        <div className="flex items-end gap-4">
          <div className="flex-1 space-y-2">
            <Label htmlFor="endpoint-url">IP</Label>
            <Input
              id="endpoint-ip"
              value={newEndpoint.ip}
              onChange={(e) => setNewEndpoint({ ...newEndpoint, ip: e.target.value })}
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
                      onClick={() => handleDeleteEndpoint(endpoint.ip, endpoint.port)}
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
