import DiskAreaChart from "@/features/metrics/components/disks/disk.area-chart";
import { Box, Card, Center, Space, Stack, Text } from "@mantine/core";
import { Enumerable } from "@/hooks/useServerEventsEnumerableStore";
import { Disk } from "@/lib/types";

interface DiskInfoProps {
  disk: Enumerable<Disk>;
}
const DiskInfo: React.FC<DiskInfoProps> = ({ disk}) => {
  return (
    <Card shadow="xl" p="sm" radius={"md"} withBorder>
      <Stack spacing="xl">
        <Box style={{ height: "300px" }}>
          <DiskAreaChart key={disk.id} disk={disk} />
        </Box>
        <Center>
          <Stack spacing={"xs"}>
            <Text>Name: {disk.data.at(-1)?.name}</Text>
            <Text>Mount Point: {disk.data.at(-1)?.mountPoint}</Text>
          </Stack>
          <Space style={{ width: "120px" }} />
          <Stack spacing={"xs"}>
            <Text>File System: {disk.data.at(-1)?.fileSystem}</Text>
            <Text>Removable: {disk.data.at(-1)?.isRemovable.toString()}</Text>
          </Stack>
        </Center>
      </Stack>
    </Card>
  );
};

export default DiskInfo;
