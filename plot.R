# library
library(ggplot2)
library(tidyr)

# Define your color palette (for continuous scale)
my_palette4 <- c("#bf616a", "#5e81ac", "#ebcb8b", "#a3be8c")
my_palette6 <- c("#bf616a", "#5e81ac", "#808080","#a3be8c","#ebcb8b","#b48ead")
my_palette7 <- c("#bf616a", "#5e81ac", "#808080","#a3be8c","#ebcb8b","#b48ead","#88c0d0")


# =========== PLOT THREADS =============
data <- read.csv("simu-out/threads.csv", header = TRUE)

plot <- ggplot(data, aes(x = time, y = threads)) +
  geom_line(color = "#5e81ac", linewidth = 0.5) +
  labs(x = "Cycles", y = "Threads", title = "Working Threads Over Time") +
  theme(
    plot.title = element_text(size = 12, hjust = 0.5),  # Smaller & centered title
    axis.title = element_text(size = 10),               # Smaller axis titles
    axis.text = element_text(size = 10),               # Smaller tick labels
    
    # Remove minor gridlines (keep major if needed)
    panel.grid.minor = element_blank(),                 
    panel.grid.major = element_line(color = "gray90"),  # Lighter major grid
    
    # Black border around the plot
    panel.border = element_rect(color = "black", fill = NA, linewidth = 0.8),
    
    panel.background = element_rect(fill = "white"),
    axis.ticks.length = unit(-0.15, "cm"),  # Negative value flips ticks inward
  )

ggsave("simu-out/working-threads.pdf", width = 10, height = 5, units = "cm")

# =========== PLOT BUSY RATES =============
data1 <- read.csv("simu-out/red-rate.csv", header = TRUE)
data2 <- read.csv("simu-out/alu-rate.csv", header = TRUE)
data1$source <- "Reducer"  # Label for the first dataset
data2$source <- "ALU"  # Label for the second dataset
combined_data <- rbind(data1, data2)  # Stack them vertically

plot <- ggplot(combined_data, aes(x = time, y = rate, color = source)) +
  geom_line(linewidth = 0.5) +
  labs(x = "Cycles", y = "Busy Rate (%)", title = "Components Busy Rate Over Time") +
  scale_color_manual(values = c("Reducer" = "#5e81ac", "ALU" = "#bf616a")) +
  guides(color = guide_legend(position = "inside"))+
  theme(
    plot.title = element_text(size = 12, hjust = 0.5),  # Smaller & centered title
    axis.title = element_text(size = 10),               # Smaller axis titles
    axis.text = element_text(size = 10),               # Smaller tick labels
    
    # Remove minor gridlines (keep major if needed)
    panel.grid.minor = element_blank(),                 
    panel.grid.major = element_line(color = "gray90"),  # Lighter major grid
    
    # Black border around the plot
    panel.border = element_rect(color = "black", fill = NA, linewidth = 0.8),
    
    # Legend inside plot with black border
    legend.position.inside = c(0.65, 0.1),
    legend.direction = "horizontal",  # Key change: horizontal layout
    legend.box.just = "center",      # Centers items in the legend box
    legend.text = element_text(size = 8, margin = margin(r = 1)),  # Smaller legend text
    legend.background = element_rect(
      color = "black",  # Black border
      fill = "white",   # White background
      linewidth = 0.3   # Border thickness
    ),

    legend.margin = margin(1, 2, 1, 2),  # Tight internal padding (top,right,bottom,left)
    
    # Remove legend title
    legend.title = element_blank(),
    
    panel.background = element_rect(fill = "white"),
    axis.ticks.length = unit(-0.15, "cm"),  # Negative value flips ticks inward
  )

ggsave("simu-out/busy-rate.pdf", width = 10, height = 6, units = "cm")

# Simulate some data
data <- read.csv("simu-out/buffer-util.csv", header = TRUE)

# Reshape and plot
data_long <- pivot_longer(data, cols = -time, names_to = "variable", values_to = "value")
ggplot(data_long, aes(x = time, y = value)) +
  geom_line(color = "#5e81ac", linewidth = 0.5)+
  facet_wrap(~ variable, ncol = 3, scales = "free_y") +
  scale_y_continuous(limits = c(0, 8)) +  # Force y-axis range
  labs(x = "Cycle", y = "Buffer Depth") +
  theme_bw()
ggsave("simu-out/buffer-util.pdf", width = 20, height = 16, units = "cm")
