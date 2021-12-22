library ieee;
use ieee.numeric_std.all;

--------------------------------------------------

package data_types is
	type direction_t is (forward, down, up);
	type command_t is record
		direction : direction_t;
		amount    : unsigned(7 downto 0);
	end record;
end package;

--------------------------------------------------
--------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.data_types.all;

--------------------------------------------------

entity InputDataROM is
	port
	(
		CLK     : in  std_logic;
		CLRn    : in  std_logic := '1';
		ADDR    : in  unsigned(10 downto 0);
		COMMAND : out command_t
	);
end entity;

--------------------------------------------------

architecture InputDataROM_arch of InputDataROM is
	type mem_command_t is record
		direction : direction_t;
		amount    : integer;
	end record;
	type memory_t is array(0 to 2 ** ADDR'length - 1) of mem_command_t;
	constant rom : memory_t := (
		(forward, 2),  (forward, 5),  (forward, 5),  (down, 4),  (forward, 8),   (down, 1),   (forward, 2),  (forward, 2),
		(down, 9), (down, 2), (forward, 6),  (forward, 3),  (down, 9),  (down, 2),  (forward, 4),  (up, 2),  (forward, 7),
		(up, 6), (forward, 8), (forward, 4), (up, 5), (forward, 3), (down, 9), (forward, 5),  (forward, 8),  (forward, 6),
		(forward, 6), (forward, 8), (forward, 8), (forward, 5), (down, 2), (forward, 8), (down, 4), (down, 6),  (down, 6),
		(forward, 7), (forward, 6), (up, 3), (up, 5), (down, 5), (down, 3), (up, 2), (down, 7),  (down, 2),  (forward, 1),
		(forward, 6),  (up, 1),  (down, 7),  (forward, 8),  (down, 7),  (down, 6),  (forward, 5),  (up, 6),  (forward, 9),
		(down, 9), (up, 3), (down, 6),  (down, 1),  (forward, 6),  (up, 1),  (down, 1),  (up, 3),  (up, 7),  (forward, 1),
		(down, 5),  (forward, 7),  (forward, 3),  (down, 9),  (up, 8),  (forward, 1),  (down, 4),  (up, 1),  (forward, 9),
		(forward, 7),  (down, 2),  (forward, 8),  (down, 2),  (down, 5),  (down, 2),  (up, 1),  (down, 5),   (forward, 4),
		(forward, 8), (forward, 3), (forward, 3), (forward, 1), (down, 1), (forward, 1),  (down, 7),  (up, 7),  (down, 3),
		(down, 1),  (down, 7),  (down, 4),  (forward, 7),  (forward, 5),  (down, 9),  (down, 5),  (forward, 3),   (up, 7),
		(down, 2),  (up, 1),  (forward, 1),  (forward, 2),  (down, 7),  (down, 3),  (forward, 7),  (up, 4),  (forward, 3),
		(up, 8), (down, 9), (down, 4), (forward, 5), (forward, 6), (down, 3), (down, 5), (down, 4), (down, 9),  (down, 8),
		(forward, 6), (down, 3), (down, 1),  (down, 9),  (down, 6),  (forward, 9),  (forward, 2),  (up, 5),  (forward, 5),
		(forward, 4),  (down, 9),  (up, 7),  (up, 2),  (forward, 1),  (forward, 8),  (forward, 6),   (up, 8),   (down, 9),
		(down, 2),  (forward, 7),  (down, 6),  (forward, 7),   (up, 9),   (up, 7),   (down, 8),   (up, 1),   (forward, 2),
		(forward, 9),  (down, 9),  (forward, 6),  (down, 2),  (down, 1),  (up, 3),  (forward, 4),  (down, 3),   (down, 4),
		(down, 9), (up, 8), (up, 8), (forward, 3), (up, 7), (forward, 9), (forward, 7),  (up, 2),  (up, 8),  (forward, 2),
		(down, 6), (forward, 3), (forward, 1), (down, 7), (down, 2), (forward, 9), (forward, 9), (down, 3),  (forward, 2),
		(forward, 8), (down, 6), (forward, 6), (forward, 5), (forward, 1), (forward, 6),  (down, 8),  (down, 7),  (up, 9),
		(down, 6), (up, 7), (down, 2), (up, 8), (up, 8), (down, 3), (down, 7),  (up, 2),  (up, 2),  (down, 6),  (down, 6),
		(forward, 2), (down, 6), (forward, 8), (forward, 9), (down, 3), (forward, 6),  (down, 9),  (forward, 1),  (up, 6),
		(down, 3),  (up, 5),  (forward, 9),  (forward, 7),  (forward, 9),  (forward, 5),  (up, 6),  (down, 3),  (down, 3),
		(down, 3), (forward, 1), (up, 5), (forward, 3), (forward, 2), (down, 1),  (forward, 7),  (down, 1),  (forward, 6),
		(forward, 5), (forward, 9), (up, 6), (forward, 1), (up, 8), (down, 7), (forward, 6),  (forward, 3),  (forward, 1),
		(up, 6), (forward, 4),  (up, 6),  (down, 7),  (forward, 8),  (forward, 4),  (forward, 6),  (forward, 5),  (up, 6),
		(down, 7), (up, 9), (down, 7), (forward, 3), (down, 7), (forward, 6), (down, 6), (down, 6), (forward, 8), (up, 8),
		(forward, 8),  (forward, 2),  (down, 8),   (forward, 4),   (forward, 9),   (forward, 7),   (down, 4),   (down, 7),
		(forward, 4), (forward, 3),  (forward, 2),  (forward, 2),  (forward, 5),  (down, 6),  (forward, 1),  (forward, 6),
		(up, 8), (up, 3),  (up, 9),  (forward, 3),  (up, 8),  (forward, 3),  (up, 8),  (down, 9),  (up, 7),  (forward, 9),
		(forward, 1), (down, 7), (forward, 9),  (down, 6),  (forward, 4),  (down, 3),  (up, 9),  (forward, 5),  (down, 4),
		(up, 6), (down, 1), (up, 9), (forward, 2), (up, 2),  (down, 1),  (forward, 7),  (down, 6),  (down, 4),  (down, 7),
		(down, 4), (down, 8), (up, 9), (down, 6), (up, 3), (down, 7), (forward, 3), (forward, 1), (down, 1), (forward, 2),
		(forward, 6), (down, 6), (forward, 5), (down, 8), (down, 8),  (down, 6),  (down, 9),  (forward, 6),  (forward, 7),
		(up, 8), (forward, 5), (up, 8), (down, 6), (up, 3), (forward, 9), (up, 9),  (forward, 2),  (up, 6),  (forward, 2),
		(down, 8),  (forward, 3),  (down, 5),  (down, 8),  (up, 6),   (down, 3),   (down, 7),   (down, 4),   (forward, 5),
		(forward, 8),  (down, 8),  (forward, 6),  (down, 8),  (up, 1),  (forward, 5),   (down, 3),   (down, 2),   (up, 1),
		(forward, 3), (forward, 7), (forward, 6), (forward, 9), (up, 8), (down, 2),  (down, 1),  (down, 7),  (forward, 4),
		(forward, 2), (down, 9), (down, 3), (down, 5), (up, 6),  (down, 2),  (forward, 8),  (up, 1),  (up, 5),  (down, 6),
		(down, 7), (forward, 3), (down, 4), (up, 6),  (down, 9),  (up, 6),  (down, 2),  (down, 2),  (down, 6),  (down, 3),
		(forward, 9), (down, 6), (forward, 5), (forward, 6), (forward, 4), (down, 6),  (forward, 3),  (down, 9),  (up, 8),
		(forward, 2), (up, 3), (forward, 1), (up, 8),  (forward, 1),  (down, 6),  (down, 3),  (forward, 5),  (forward, 4),
		(down, 9),  (up, 3),  (down, 6),  (forward, 4),  (down, 8),  (down, 3),  (down, 7),  (forward, 1),   (forward, 2),
		(forward, 8), (down, 2), (down, 6), (up, 9), (forward, 2), (forward, 4),  (forward, 8),  (down, 3),  (forward, 9),
		(down, 7), (up, 8), (forward, 1),  (down, 6),  (down, 3),  (forward, 5),  (forward, 5),  (forward, 2),  (down, 2),
		(down, 2), (down, 2), (up, 4), (forward, 2), (forward, 5), (forward, 1),  (forward, 5),  (down, 3),  (forward, 8),
		(up, 1), (forward, 9), (up, 5), (forward, 2), (down, 4), (forward, 8), (forward, 9), (up, 6), (up, 1),  (down, 7),
		(down, 3), (forward, 9), (down, 3),  (down, 3),  (forward, 3),  (forward, 6),  (up, 4),  (forward, 2),  (down, 9),
		(forward, 5), (forward, 3), (forward, 5), (down, 6), (up, 6), (down, 7),  (forward, 2),  (down, 4),  (forward, 6),
		(forward, 5), (forward, 8), (down, 4), (up, 3), (up, 4), (down, 9), (forward, 9), (down, 7), (down, 1), (down, 9),
		(down, 6), (down, 8), (forward, 7), (down, 3), (up, 3), (up, 5), (forward, 5), (up, 6),  (down, 6),  (forward, 9),
		(down, 7),  (forward, 6),  (up, 1),  (forward, 7),  (forward, 8),  (down, 9),  (down, 1),  (forward, 4),  (up, 2),
		(down, 3), (up, 3), (down, 6),  (forward, 6),  (down, 7),  (down, 2),  (forward, 8),  (forward, 2),  (forward, 3),
		(forward, 5), (up, 8), (up, 6),  (down, 1),  (up, 7),  (down, 1),  (down, 8),  (forward, 5),  (up, 7),  (down, 1),
		(forward, 8), (down, 6), (down, 2), (up, 7), (down, 1), (forward, 1), (up, 8), (forward, 5), (down, 7), (down, 8),
		(forward, 4), (down, 6),  (down, 2),  (forward, 4),  (forward, 7),  (down, 5),  (down, 1),  (down, 8),  (down, 3),
		(up, 8),  (down, 6),  (down, 1),  (down, 1),  (forward, 4),  (down, 8),   (up, 5),   (forward, 3),   (forward, 3),
		(forward, 5), (forward, 8), (up, 7), (forward, 5),  (down, 8),  (forward, 9),  (up, 9),  (forward, 1),  (down, 2),
		(up, 9),  (down, 7),  (up, 1),  (up, 3),  (forward, 9),  (forward, 8),  (forward, 3),   (forward, 8),   (down, 8),
		(down, 5), (down, 7), (up, 7), (forward, 9), (up, 8), (down, 9), (down, 9), (down, 1), (up, 4), (up, 4),  (up, 3),
		(forward, 3),  (down, 7),  (down, 3),  (forward, 5),  (up, 9),  (down, 3),  (up, 2),  (forward, 1),  (forward, 4),
		(up, 9),  (forward, 6),  (down, 6),  (down, 6),  (forward, 2),  (forward, 2),  (down, 4),  (forward, 9),  (up, 6),
		(down, 8), (down, 8),  (down, 9),  (up, 4),  (down, 9),  (down, 3),  (down, 7),  (forward, 4),  (up, 4),  (up, 5),
		(down, 2),  (up, 2),  (forward, 1),  (up, 8),  (forward, 2),  (down, 6),  (forward, 9),  (down, 9),  (forward, 9),
		(up, 8), (down, 5), (up, 3), (up, 9), (down, 8), (forward, 6), (forward, 2), (forward, 7),  (down, 9),  (down, 8),
		(up, 9), (down, 5), (up, 5), (down, 1),  (forward, 3),  (down, 2),  (down, 5),  (down, 4),  (down, 1),  (down, 5),
		(down, 5), (down, 5), (forward, 8), (down, 2),  (down, 3),  (down, 4),  (down, 2),  (up, 8),  (up, 1),  (down, 5),
		(forward, 8), (down, 6), (forward, 9), (down, 6), (down, 6), (down, 5), (forward, 5), (forward, 3),  (forward, 7),
		(down, 6),  (forward, 4),  (down, 8),  (up, 9),  (up, 6),  (forward, 5),  (down, 4),  (forward, 8),  (forward, 1),
		(forward, 8),  (up, 5),  (up, 5),  (forward, 6),   (down, 3),   (down, 7),   (up, 7),   (down, 5),   (forward, 1),
		(forward, 3), (up, 3), (down, 3), (down, 3), (down, 5), (forward, 9), (down, 2), (up, 5),  (up, 2),  (forward, 1),
		(down, 1),  (forward, 5),  (down, 8),  (forward, 8),  (forward, 9),  (forward, 3),   (forward, 2),   (forward, 4),
		(down, 5),  (up, 1),  (down, 5),  (up, 1),  (up, 4),  (forward, 2),  (forward, 4),  (down, 6),  (up, 2),  (up, 4),
		(forward, 4),  (down, 7),  (forward, 2),  (up, 1),  (forward, 4),  (up, 4),  (forward, 9),  (down, 1),  (down, 4),
		(up, 1), (forward, 2), (forward, 3), (down, 6),  (forward, 9),  (forward, 3),  (down, 2),  (forward, 9),  (up, 4),
		(forward, 5),  (forward, 2),  (down, 2),  (forward, 5),  (down, 5),  (down, 3),  (down, 7),  (down, 7),   (up, 3),
		(down, 7),  (forward, 9),  (forward, 7),  (up, 4),  (forward, 7),  (up, 8),  (down, 8),  (forward, 5),  (down, 2),
		(down, 9), (up, 8), (forward, 4), (forward, 4),  (forward, 2),  (up, 4),  (down, 5),  (forward, 3),  (forward, 6),
		(forward, 5), (forward, 3), (forward, 7), (up, 9), (forward, 1), (forward, 2), (up, 7),  (down, 4),  (forward, 8),
		(down, 2),  (forward, 1),  (down, 1),  (down, 9),  (up, 2),  (down, 2),   (forward, 8),   (forward, 5),   (up, 9),
		(forward, 5),  (down, 2),  (forward, 2),   (down, 7),   (forward, 5),   (down, 1),   (forward, 9),   (forward, 8),
		(forward, 8), (forward, 9), (forward, 8),  (down, 9),  (forward, 8),  (down, 1),  (down, 2),  (down, 9),  (up, 2),
		(forward, 8), (forward, 8), (down, 9), (forward, 8),  (up, 8),  (forward, 2),  (down, 5),  (up, 3),  (forward, 2),
		(up, 3), (down, 6), (forward, 5), (up, 7),  (forward, 2),  (forward, 7),  (forward, 7),  (down, 8),  (forward, 3),
		(up, 5), (down, 8), (down, 3), (down, 8), (up, 6), (down, 6), (up, 4), (forward, 7), (up, 6), (up, 1),  (down, 5),
		(up, 8), (forward, 8), (forward, 9),  (down, 8),  (forward, 6),  (down, 8),  (down, 9),  (down, 8),  (forward, 9),
		(up, 1), (down, 1), (down, 8), (down, 5), (down, 2), (up, 7), (forward, 4),  (down, 4),  (forward, 6),  (down, 1),
		(forward, 5),  (forward, 3),  (forward, 9),  (up, 8),  (down, 4),  (down, 6),   (down, 4),   (down, 1),   (up, 9),
		(forward, 9), (down, 9), (forward, 7), (down, 2), (down, 8),  (forward, 2),  (forward, 5),  (down, 6),  (down, 1),
		(down, 1), (up, 3), (forward, 9), (up, 7), (down, 1),  (down, 6),  (forward, 6),  (up, 2),  (down, 4),  (down, 7),
		(forward, 3),  (down, 9),  (down, 1),   (forward, 7),   (forward, 2),   (forward, 6),   (down, 4),   (forward, 7),
		(forward, 8), (forward, 3), (forward, 7), (up, 8), (forward, 5), (up, 8), (down, 8),  (forward, 3),  (forward, 8),
		(forward, 7), (forward, 3), (down, 4), (forward, 9), (forward, 4), (up, 6), (forward, 4),  (forward, 6),  (up, 4),
		(forward, 6), (forward, 2), (down, 3), (down, 3), (down, 4), (down, 1), (down, 9),  (up, 1),  (up, 9),  (down, 7),
		(up, 9), (forward, 5), (down, 2), (up, 7), (forward, 9), (up, 1),  (down, 5),  (up, 8),  (forward, 9),  (down, 1),
		(up, 3), (forward, 3), (forward, 9), (up, 7), (forward, 3), (down, 7), (forward, 3),  (forward, 5),  (forward, 4),
		(up, 3), (forward, 3), (down, 8), (forward, 7), (up, 5), (forward, 9), (down, 6), (up, 6), (up, 1),  (forward, 8),
		(down, 9), (forward, 9), (forward, 9), (down, 5), (down, 7),  (forward, 7),  (down, 5),  (down, 3),  (forward, 4),
		(up, 6), (forward, 6), (forward, 2), (down, 8), (down, 6), (forward, 1),  (forward, 1),  (down, 6),  (forward, 1),
		(down, 6),  (down, 2),  (forward, 1),  (forward, 6),   (up, 4),   (up, 7),   (up, 3),   (down, 6),   (forward, 1),
		(forward, 9), (forward, 4), (down, 4), (forward, 3), (down, 8), (down, 6), (down, 6), (forward, 2),  (forward, 8),
		(forward, 6), (down, 4), (up, 2), (down, 1), (up, 4),  (down, 8),  (forward, 9),  (down, 4),  (down, 8),  (up, 8),
		(forward, 2),  (up, 9),  (down, 9),  (forward, 2),  (down, 8),  (forward, 3),  (down, 1),  (up, 9),  (forward, 6),
		(down, 9),  (forward, 2),  (down, 2),  (up, 5),  (down, 4),   (down, 5),   (down, 7),   (forward, 4),   (down, 4),
		(forward, 6),  (forward, 2),  (down, 8),  (up, 3),  (up, 9),  (forward, 3),  (forward, 6),  (down, 6),  (down, 2),
		(down, 5),  (down, 4),  (forward, 1),  (up, 2),  (down, 3),  (forward, 6),  (forward, 5),  (down, 4),   (down, 3),
		(forward, 2), (forward, 4), (down, 9), (down, 3), (down, 4), (up, 6),  (forward, 8),  (up, 2),  (up, 8),  (up, 4),
		(forward, 7),  (forward, 5),  (forward, 9),  (down, 6),   (forward, 7),   (forward, 9),   (up, 3),   (forward, 6),
		others => (direction_t'low, 0)
	);
begin
	process (CLK)
	begin
		if rising_edge(CLK) then
			COMMAND <= (direction_t'low, (others => '0'));
			if CLRn = '1' then
				COMMAND.direction <= rom(to_integer(ADDR)).direction;
				COMMAND.amount    <= to_unsigned(rom(to_integer(ADDR)).amount, COMMAND.amount'length);
			end if;
		end if;
	end process;
end InputDataROM_arch;

--------------------------------------------------
--------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.data_types.all;

--------------------------------------------------

entity AoC_20211202 is
	port (
		CLK      : in  std_logic;
		CLRn     : in  std_logic := '1';
		DONE     : out std_logic;
		RESULT_1 : out unsigned(31 downto 0);
		RESULT_2 : out unsigned(31 downto 0)
	);
end entity;

--------------------------------------------------

architecture AoC_20211202_arch of AoC_20211202 is
	signal addr     : unsigned(10 downto 0) := (others => '0');
	signal command  : command_t;
	signal done_bit : std_logic := '0';

	type position_t is record
		x: unsigned(23 downto 0);
		y: unsigned(23 downto 0);
	end record;
	signal position_1 : position_t := (others => (others => '0'));
	signal position_2 : position_t := (others => (others => '0'));
	signal aim : unsigned(15 downto 0) := (others => '0');
begin

	idr : entity work.InputDataROM port map (CLK => CLK, CLRn => CLRn, ADDR => addr, COMMAND => command);

	process (CLK)
	begin
		if rising_edge(CLK) then
			done_bit <= '0';
			if CLRn = '0' then
				addr <= (others => '0');
				DONE <= '0';
			else
				if addr < 1000 then
					addr <= addr + 1;
				else
					done_bit <= '1';
				end if;
				DONE <= done_bit;
			end if;
		end if;
	end process;

	process (CLK)
	begin
		if rising_edge(CLK) then
			if CLRn = '0' then
				position_1 <= (others => (others => '0'));
				RESULT_1   <= (others => '0');
			else
				if done_bit = '1' then
					RESULT_1 <= resize(position_1.x * position_1.y, RESULT_1'length);
				else
					case command.direction is
						when forward => position_1.x <= position_1.x + command.amount;
						when down    => position_1.y <= position_1.y + command.amount;
						when up      => position_1.y <= position_1.y - command.amount;
					end case;
				end if;
			end if;
		end if;
	end process;

	process (CLK)
	begin
		if rising_edge(CLK) then
			if CLRn = '0' then
				position_2 <= (others => (others => '0'));
				RESULT_2   <= (others => '0');
			else
				if done_bit = '1' then
					RESULT_2 <= resize(position_2.x * position_2.y, RESULT_2'length);
				else
					case command.direction is
						when forward => position_2.x <= position_2.x + command.amount;
						                position_2.y <= position_2.y + resize(aim * command.amount, position_2.y'length);
						when down    => aim <= aim + command.amount;
						when up      => aim <= aim - command.amount;
					end case;
				end if;
			end if;
		end if;
	end process;

end AoC_20211202_arch;

--------------------------------------------------
--------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

--------------------------------------------------

entity AoC_20211202_tb is
end entity;

--------------------------------------------------

architecture AoC_20211202_tb_arch of AoC_20211202_tb is
	constant clk_period : time := 20 ns;
	signal clk      : std_logic := '0';
	signal clrn     : std_logic := '0';
	signal done     : std_logic := '0';
	signal result_1 : unsigned(31 downto 0) := (others => '0');
	signal result_2 : unsigned(31 downto 0) := (others => '0');
begin
	clk  <= not clk after clk_period / 2 when done /= '1' else '1';
	clrn <= '0', '1' after 2 * clk_period;
	aoc : entity work.AoC_20211202 port map (CLK => clk, CLRn => clrn, DONE => done, RESULT_1 => result_1, RESULT_2 => result_2);
end;
