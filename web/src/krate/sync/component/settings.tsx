// components/EndpointSettings.tsx
import { useState } from "react";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
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

type Endpoint = {
  id: string;
  name: string;
  url: string;
  syncInterval: string;
  lastSynced: string;
};

export const EndpointSettings = () => {
  const [endpoints, setEndpoints] = useState<Endpoint[]>([
    {
      id: "1",
      name: "Production API",
      url: "https://api.example.com",
      syncInterval: "daily",
      lastSynced: "2023-05-15",
    },
    {
      id: "2",
      name: "Staging API",
      url: "https://staging.api.example.com",
      syncInterval: "hourly",
      lastSynced: "2023-05-15",
    },
  ]);
  const [newEndpoint, setNewEndpoint] = useState<
    Omit<Endpoint, "id" | "lastSynced">
  >({
    name: "",
    url: "",
    syncInterval: "daily",
  });

  const handleAddEndpoint = () => {
    if (!newEndpoint.name || !newEndpoint.url) return;

    setEndpoints([
      ...endpoints,
      {
        ...newEndpoint,
        id: Date.now().toString(),
        lastSynced: new Date().toISOString().split("T")[0],
      },
    ]);
    setNewEndpoint({ name: "", url: "", syncInterval: "daily" });
  };

  const handleDeleteEndpoint = (id: string) => {
    setEndpoints(endpoints.filter((ep) => ep.id !== id));
  };

  const handleSyncNow = (id: string) => {
    setEndpoints(
      endpoints.map((ep) =>
        ep.id === id
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
            <Label htmlFor="endpoint-name">Endpoint Name</Label>
            <Input
              id="endpoint-name"
              value={newEndpoint.name}
              onChange={(e) =>
                setNewEndpoint({ ...newEndpoint, name: e.target.value })
              }
              placeholder="Enter endpoint name"
            />
          </div>

          <div className="flex-1 space-y-2">
            <Label htmlFor="endpoint-url">URL</Label>
            <Input
              id="endpoint-url"
              value={newEndpoint.url}
              onChange={(e) =>
                setNewEndpoint({ ...newEndpoint, url: e.target.value })
              }
              placeholder="https://example.com/api"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="sync-interval">Sync Interval</Label>
            <Select
              value={newEndpoint.syncInterval}
              onValueChange={(value) =>
                setNewEndpoint({ ...newEndpoint, syncInterval: value })
              }
            >
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="Select interval" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="hourly">Hourly</SelectItem>
                <SelectItem value="daily">Daily</SelectItem>
                <SelectItem value="weekly">Weekly</SelectItem>
                <SelectItem value="monthly">Monthly</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <Button onClick={handleAddEndpoint}>Add Endpoint</Button>
        </div>

        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>URL</TableHead>
              <TableHead>Sync Interval</TableHead>
              <TableHead>Last Synced</TableHead>
              <TableHead>Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {endpoints.map((endpoint) => (
              <TableRow key={endpoint.id}>
                <TableCell>{endpoint.name}</TableCell>
                <TableCell>{endpoint.url}</TableCell>
                <TableCell className="capitalize">
                  {endpoint.syncInterval}
                </TableCell>
                <TableCell>{endpoint.lastSynced}</TableCell>
                <TableCell>
                  <div className="flex gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleSyncNow(endpoint.id)}
                    >
                      Sync Now
                    </Button>
                    <Button
                      variant="destructive"
                      size="sm"
                      onClick={() => handleDeleteEndpoint(endpoint.id)}
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
