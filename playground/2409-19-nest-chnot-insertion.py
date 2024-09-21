import uuid
from typing import List
from pydantic import BaseModel

rel_dict = {}
child_dict = {}
next_dict = {}


def uid(suffix: int | None = None):
    return str(uuid.uuid4()).replace("-", "") + (
        ("-" + str(suffix)) if suffix is not None else ""
    )


class Chnot(BaseModel):
    suffix: int
    meta_id: str
    tree_id: str

    ids: List[str]
    prev_meta_id: str | None
    parent_meta_id: str | None

    perm_content: str

    @staticmethod
    def new(
        suffix: int,
        parent_meta_id: str | None,
        prev_meta_id: str | None,
        perm_content: str,
        tree_id: str,
    ) -> "Chnot":
        chnot = Chnot(
            suffix=suffix,
            meta_id=uid(suffix=suffix),
            tree_id=tree_id,
            ids=[uid(suffix=suffix)],
            parent_meta_id=parent_meta_id,
            prev_meta_id=prev_meta_id,
            perm_content=perm_content,
        )

        return chnot

    def newest_id(self) -> str:
        return self.ids[len(self.ids) - 1]

    def content(self) -> str:
        return self.perm_content + "’".join(["" for x in self.ids])

    def prev_id(self) -> str:
        return (
            f"'{rel_dict[self.prev_meta_id].newest_id()}'"
            if self.prev_meta_id is not None
            else "null"
        )

    def parent_id(self) -> str:
        return (
            f"'{rel_dict[self.parent_meta_id].newest_id()}'"
            if self.parent_meta_id is not None
            else "null"
        )

    def new_version(self):
        self.ids.append(uid(self.suffix))


for suffix in range(0, 1000):
    prev_meta_id = None
    parent_meta_id = None
    tree_id = uid(suffix)
    for i in range(ord("A"), ord("E") + 1):
        l1 = f"{chr(i)}"
        parent_meta_id = None
        c1 = Chnot.new(
            suffix=suffix,
            parent_meta_id=parent_meta_id,
            prev_meta_id=prev_meta_id,
            perm_content=l1,
            tree_id=tree_id,
        )
        rel_dict[c1.meta_id] = c1
        prev_meta_id = None
        parent_meta_id = c1.meta_id
        for j in range(ord("A"), ord("C") + 1):
            l2 = f"{l1}{chr(j)}"
            c2 = Chnot.new(
                suffix=suffix,
                parent_meta_id=parent_meta_id,
                prev_meta_id=prev_meta_id,
                perm_content=l2,
                tree_id=tree_id,
            )
            rel_dict[c2.meta_id] = c2
            prev_meta_id = None
            parent_meta_id = c2.meta_id

            for k in range(ord("A"), ord("C") + 1):
                l3 = f"{l2}{chr(k)}"
                c3 = Chnot.new(
                    suffix=suffix,
                    parent_meta_id=parent_meta_id,
                    prev_meta_id=prev_meta_id,
                    perm_content=l3,
                    tree_id=tree_id,
                )
                rel_dict[c3.meta_id] = c3
                prev_meta_id = c3.meta_id
            prev_meta_id = c2.meta_id
        prev_meta_id = c1.meta_id

for c in rel_dict.values():
    if c.prev_meta_id is not None:
        next_dict[c.prev_meta_id] = c.meta_id
    if c.parent_meta_id is not None:
        child_dict.setdefault(c.parent_meta_id, []).append(c.meta_id)


def chnot_sql(c: Chnot):
    return f"INSERT INTO public.chnots(id, meta_id, tree_id, \"content\", \"type\", \"namespace\", delete_time, insert_time) VALUES('{c.newest_id()}', '{c.meta_id}', '{c.tree_id}', '{c.content()}', 'mwdt', 'public', null, CURRENT_TIMESTAMP);"


def chnot_hierarchy_sql(c: Chnot):
    return f"INSERT INTO public.chnot_hierarchies(id, chnot_id, prev_id, parent_id, insert_time) VALUES ('{uid()}', '{c.newest_id()}', {c.prev_id()}, {c.parent_id()}, CURRENT_TIMESTAMP);"


def chnot_new_version(c: Chnot):
    l = []
    c.new_version()
    l.append(chnot_sql(c))
    l.append(chnot_hierarchy_sql(c))
    if next_dict.get(c.meta_id) is not None:
        l.append(chnot_hierarchy_sql(rel_dict[next_dict[c.meta_id]]))
    if child_dict.get(c.meta_id) is not None:
        l.extend(chnot_hierarchy_sql(rel_dict[x]) for x in child_dict[c.meta_id])

    return l


with open("./2409-19-nest-chnot-insertion.sql", "w+") as f:
    for c in rel_dict.values():
        f.write("\n")
        f.write(chnot_sql(c))
        f.write("\n")
        f.write(chnot_hierarchy_sql(c))
    for name in ["AA", "ACB"]:
        for c in rel_dict.values():
            if not c.perm_content == name:
                continue
            l = chnot_new_version(c)
            for line in l:
                f.write(line + "\n")

    for name in ["AB", "AAC", "ADA"]:
        for c in rel_dict.values():
            if not c.perm_content == name:
                continue
            l = chnot_new_version(c)
            for line in l:
                f.write(line + "\n")
