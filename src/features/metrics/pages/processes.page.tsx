import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { Process } from "@/lib/types";
import { useState, useEffect, memo } from "react";
import { Button, TextInput } from "@mantine/core";
import { IconSearch } from "@tabler/icons";
import { useDebouncedValue } from "@mantine/hooks";

import sortBy from "lodash.sortby";
import useMetricsContext from "@/features/metrics/hooks/useMetricsContext";
import PageWrapper from "@/components/page-wrapper";
import formatBytes from "@/features/metrics/utils/format-bytes";

interface ProcessesTableProps {
  processes: Process[];
}

const ProcessesTable: React.FC<ProcessesTableProps> = memo(({ processes }) => {
  const [query, setQuery] = useState("");
  const [debouncedQuery] = useDebouncedValue(query, 1000);
  const [records, setRecords] = useState(sortBy(processes, "name"));
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus>({
    columnAccessor: "name",
    direction: "asc",
  });

  useEffect(() => {
    const filteredRecords = processes.filter((process) => {
      const filteredName = process.name
        .toLowerCase()
        .includes(debouncedQuery.toLowerCase());
      const filteredPid = process.pid
        .toString()
        .toLowerCase()
        .includes(debouncedQuery.toLowerCase());

      return filteredName || filteredPid;
    });
    setRecords(sortBy(filteredRecords, sortStatus.columnAccessor));
  }, [debouncedQuery]);

  useEffect(() => {
    const data = sortBy(processes, sortStatus.columnAccessor);
    setRecords(sortStatus.direction === "desc" ? data.reverse() : data);
  }, [sortStatus]);

  return (
    <>
      <TextInput
        placeholder="Search Process..."
        icon={<IconSearch size={16} />}
        value={query}
        onChange={(e) => setQuery(e.currentTarget.value)}
      />
      <DataTable
        striped
        highlightOnHover
        records={records}
        idAccessor="pid"
        sortStatus={sortStatus}
        onSortStatusChange={setSortStatus}
        minHeight={"calc(100vh - 190px)"}
        height={"90vh"}
        columns={[
          {
            accessor: "pid",
            textAlignment: "right",
            title: "PID",
            sortable: true,
          },
          { accessor: "name", sortable: true },
          {
            accessor: "cpuUsage",
            render: ({ cpuUsage }) => `${Math.round(cpuUsage * 100) / 100}%`,
            sortable: true,
            title: "CPU Usage",
          },
          {
            accessor: "memoryUsage",
            sortable: true,
            render: ({ memoryUsage }) => formatBytes(memoryUsage),
          },
          { accessor: "status", sortable: true },
          {
            accessor: "actions",

            render: (record, index) => (
              <Button variant="outline" onClick={() => {}} color="red">
                Kill
              </Button>
            ),
          },
        ]}
      />
    </>
  );
});

const ProcessesPage = () => {
  const { processes } = useMetricsContext();

  return (
    <PageWrapper name="Processes">
      <ProcessesTable processes={processes} />
    </PageWrapper>
  );
};

export default ProcessesPage;
