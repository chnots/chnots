import { useState, useEffect } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { Button } from "@/common/component/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogTrigger,
  DialogClose,
} from "@/common/component/ui/dialog";
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
import { Switch } from "@/common/component/ui/switch";
import { Checkbox } from "@/common/component/ui/checkbox";
import { Badge } from "@/common/component/ui/badge";
import { Trash2, PlusCircle, Pencil } from "lucide-react";
import { genTID } from "@/lib/id_util";
import { deleteKSpace, overwriteKSpace } from "../service";
import { useKSpaceStore } from "../store";

// Form validation schema
const kspaceSchema = z.object({
  name: z.string().min(1, "Name is required"),
  color: z.string().min(1, "Color is required"),
  managers: z.array(z.string()),
  public_access: z.boolean(),
});

type KSpaceFormValues = z.infer<typeof kspaceSchema>;

export default function KSpaceSettings() {
  const { kspaceMap, refreshKSpaces } = useKSpaceStore((store) => {
    return {
      refreshKSpaces: store.refreshKSpaces,
      kspaceMap: store.kspaceMapByName,
    };
  });

  useEffect(() => {
    refreshKSpaces();
  }, []);

  const sortedKSpaces = kspaceMap.values().toArray();
  sortedKSpaces.sort((s1, s2) => {
    return s1.name.localeCompare(s2.name);
  });

  const [editingKSpaceName, setEditingKSpace] = useState<string | undefined>(
    undefined
  );
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const preservedNames = new Set(["private", "work", "public"]);
  const form = useForm<KSpaceFormValues>({
    resolver: zodResolver(kspaceSchema),
    defaultValues: {
      name: "",
      color: "#3b82f6",
      managers: [],
      public_access: false,
    },
  });

  // Reset form when opening dialog
  useEffect(() => {
    if (isDialogOpen) {
      if (editingKSpaceName) {
        const kspace = kspaceMap.get(editingKSpaceName);
        if (kspace) {
          form.reset({
            name: kspace.name,
            color: kspace.color,
            managers: kspace.managers,
            public_access: kspace.public_access,
          });
        }
      } else {
        form.reset({
          name: "",
          color: "#3b82f6",
          managers: [],
          public_access: false,
        });
      }
    }
  }, [isDialogOpen, editingKSpaceName, form]);

  const handleOverwriteKSpace = async (values: KSpaceFormValues) => {
    if (!editingKSpaceName) return;
    const kspace = { tid: genTID(), ...values };
    await overwriteKSpace({
      kspace: kspace,
    });
    refreshKSpaces();
    setIsDialogOpen(false);
  };

  const handleDeleteKSpace = async (name: string) => {
    await deleteKSpace({
      kspace_name: name,
    });
    refreshKSpaces();
  };

  const onSubmit = (values: KSpaceFormValues) => {
    handleOverwriteKSpace(values);
  };

  const handleEditClick = (spaceName?: string) => {
    setEditingKSpace(spaceName);
    setIsDialogOpen(true);
  };

  const availableManagers = kspaceMap
    .values()
    .filter((ws) => !editingKSpaceName || ws.name !== editingKSpaceName)
    .toArray();

  return (
    <div className="container mx-auto py-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">KSpace Settings</h1>
        <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
          <DialogTrigger asChild>
            <Button
              variant="default"
              className="gap-2"
              aria-label="Add new kspace"
            >
              <PlusCircle size={16} />
              Add KSpace
            </Button>
          </DialogTrigger>
          <DialogContent className="sm:max-w-md">
            <DialogHeader>
              <DialogTitle>
                {editingKSpaceName ? "Edit KSpace" : "Create New KSpace"}
              </DialogTitle>
            </DialogHeader>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="name">KSpace Name</Label>
                {editingKSpaceName ? (
                  <Badge variant={"default"} className="capitalize">
                    {editingKSpaceName}
                  </Badge>
                ) : (
                  <>
                    <Input
                      id="name"
                      {...form.register("name")}
                      placeholder="Enter kspace name"
                      aria-invalid={
                        form.formState.errors.name ? "true" : "false"
                      }
                    />
                    {form.formState.errors.name && (
                      <p className="text-red-500 text-sm">
                        {form.formState.errors.name.message}
                      </p>
                    )}
                  </>
                )}
              </div>

              <div className="space-y-2">
                <Label htmlFor="color">Color</Label>
                <div className="flex items-center gap-3">
                  <Input
                    id="color"
                    type="color"
                    className="w-16 h-10 p-1"
                    {...form.register("color")}
                    aria-label="Select kspace color"
                  />
                  <span className="text-sm text-muted-foreground">
                    {form.watch("color")}
                  </span>
                </div>
              </div>

              <div className="space-y-2">
                <Label>Managed By</Label>
                <div className="grid gap-2">
                  {availableManagers.length > 0 ? (
                    availableManagers.map((manager) => (
                      <div
                        key={manager.tid}
                        className="flex items-center gap-3"
                      >
                        <Checkbox
                          id={`manager-${manager.tid}`}
                          checked={form
                            .watch("managers")
                            .includes(manager.name)}
                          onCheckedChange={(checked) => {
                            const managers = form.getValues("managers");
                            const newManagers = checked
                              ? [...managers, manager.name]
                              : managers.filter(
                                  (name) => name !== manager.name
                                );

                            form.setValue("managers", newManagers);
                          }}
                          aria-label={`Select ${manager.name} as manager`}
                        />
                        <Label
                          htmlFor={`manager-${manager.tid}`}
                          className="flex items-center gap-2 font-normal"
                        >
                          <div
                            className="w-4 h-4 rounded-sm"
                            style={{ backgroundColor: manager.color }}
                          />
                          {manager.name}
                        </Label>
                      </div>
                    ))
                  ) : (
                    <p className="text-sm text-muted-foreground">
                      No other kspaces available
                    </p>
                  )}
                </div>
              </div>

              <div className="flex justify-between pt-2">
                <Label
                  htmlFor="access"
                  className="flex flex-col space-y-1 items-start"
                >
                  <span>Public Access</span>
                  <span className="text-xs font-normal text-muted-foreground">
                    Allow LLM to access this kspace
                  </span>
                </Label>
                <Switch
                  id="access"
                  checked={form.watch("public_access")}
                  onCheckedChange={(value) =>
                    form.setValue("public_access", value)
                  }
                  aria-label="Toggle public access"
                />
              </div>

              <DialogFooter>
                <DialogClose asChild>
                  <Button type="button" variant="outline">
                    Cancel
                  </Button>
                </DialogClose>
                <Button type="submit">
                  {editingKSpaceName ? "Save Changes" : "Create KSpace"}
                </Button>
              </DialogFooter>
            </form>
          </DialogContent>
        </Dialog>
      </div>

      <div className="border rounded-lg">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>KSpace</TableHead>
              <TableHead>Color</TableHead>
              <TableHead>Managed By</TableHead>
              <TableHead>Access</TableHead>
              <TableHead className="text-right">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {kspaceMap.size > 0 ? (
              sortedKSpaces.map((kspace) => (
                <TableRow key={kspace.tid}>
                  <TableCell className="font-medium">{kspace.name}</TableCell>
                  <TableCell>
                    <div className="flex items-center gap-2">
                      <div
                        className="w-5 h-5 rounded-sm"
                        style={{ backgroundColor: kspace.color }}
                        aria-label={`Color: ${kspace.color}`}
                      />
                      <span className="text-sm text-muted-foreground">
                        {kspace.color}
                      </span>
                    </div>
                  </TableCell>
                  <TableCell>
                    {kspace.managers.length > 0 ? (
                      <div className="flex flex-wrap gap-1">
                        {kspace.managers.map((managerId) => {
                          const manager = kspaceMap.get(managerId);
                          return manager ? (
                            <Badge
                              key={managerId}
                              variant="outline"
                              className="flex items-center gap-1"
                            >
                              <div
                                className="w-3 h-3 rounded-sm"
                                style={{ backgroundColor: manager.color }}
                                aria-hidden
                              />
                              {manager.name}
                            </Badge>
                          ) : null;
                        })}
                      </div>
                    ) : (
                      <span className="text-muted-foreground">None</span>
                    )}
                  </TableCell>
                  <TableCell>
                    <Badge
                      variant={kspace.public_access ? "default" : "secondary"}
                      className="capitalize"
                    >
                      {kspace.public_access ? "Public" : "Private"}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-right">
                    <div className="flex justify-end gap-2">
                      <Button
                        variant="outline"
                        size="icon"
                        onClick={() => handleEditClick(kspace.name)}
                        aria-label={`Edit ${kspace.name}`}
                      >
                        <Pencil className="h-4 w-4" />
                      </Button>
                      <Button
                        variant="destructive"
                        size="icon"
                        onClick={() => handleDeleteKSpace(kspace.name)}
                        aria-label={`Delete ${kspace.name}`}
                        disabled={preservedNames.has(kspace.name)}
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell colSpan={5} className="h-24 text-center">
                  No kspaces found
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
