# kindly contributed by @Wyth@mastodon.art

import vsketch
import math
import random
from typing import List, Tuple
from shapely import geometry, affinity, ops

class sketch_091523_2222_asteroids(vsketch.SketchClass):
    # Sketch parameters:
    page_side1 = vsketch.Param(9)
    page_side2 = vsketch.Param(12)
    penW = vsketch.Param(0.3, decimals=2)
    centered = vsketch.Param(False)
    inch_border = vsketch.Param(False)
    half_inch_border = vsketch.Param(False)
    avg_radius = vsketch.Param(200)
    irregularity = vsketch.Param(0.9, decimals=2)
    spikiness = vsketch.Param(0.13, decimals=2)
    num_vertices = vsketch.Param(18)
    poly_spin_smaller = vsketch.Param(False)
    times_to_reduce = vsketch.Param(40)
    rotation_angle_min = vsketch.Param(30, decimals=2)
    rotation_angle_max = vsketch.Param(110, decimals=2)
    scale_amount = vsketch.Param(0.9, decimals=2)
    poly_vor_diag = vsketch.Param(False)
    vor_num_initial_points = vsketch.Param(6)
    vor_min_levels = vsketch.Param(3)
    vor_max_levels = vsketch.Param(5)

    def draw(self, vsk: vsketch.Vsketch) -> None:
        pen = str(f"{self.penW}mm")
        vsk.penWidth(pen)
        page_size = str(f"{self.page_side1}inx{self.page_side2}")
        if self.centered:
            vsk.size(page_size)
        else:
            vsk.size(page_size, center=False)

        def generate_polygon(center: Tuple[float, float], avg_radius: float,
                             irregularity: float, spikiness: float,
                             num_vertices: int):
            irregularity *= 2 * math.pi / num_vertices
            spikiness *= avg_radius
            angle_steps = random_angle_steps(num_vertices, irregularity)

            points = []
            angle = random.uniform(0, 2 * math.pi)
            for i in range(num_vertices):
                radius = clip(random.gauss(avg_radius, spikiness), 0, 2 * avg_radius)
                point = (center[0] + radius * math.cos(angle),
                         center[1] + radius * math.sin(angle))
                points.append(point)
                angle += angle_steps[i]

            the_poly = geometry.Polygon(points)

            return the_poly

        def random_angle_steps(steps: int, irregularity: float) -> List[float]:
            angles = []
            lower = (2 * math.pi / steps) - irregularity
            upper = (2 * math.pi / steps) + irregularity
            cumsum = 0
            for i in range(steps):
                angle = random.uniform(lower, upper)
                angles.append(angle)
                cumsum += angle

            cumsum /= (2 * math.pi)
            for i in range(steps):
                angles[i] /= cumsum
            return angles

        def clip(value, lower, upper):
            return min(upper, max(value, lower))

        def spin_smaller(poly, times_to_reduce, rotation_angle, scale_amount):
            poly_list = []
            for x in range(times_to_reduce):
                if x == 0:
                    poly_list.append(poly)
                else:
                    new_poly = affinity.scale(poly, scale_amount, scale_amount)
                    new_poly = affinity.rotate(new_poly, rotation_angle)
                    new_poly = new_poly.intersection(poly).buffer(0)
                    poly_list.append(new_poly)
                    poly = new_poly
            return geometry.MultiPolygon(poly_list)

        def vor_diag(poly, num_points: int, min_iterations: int, max_iterations: int, distribution='uniform'):
            points_list = []
            poly_list = []
            for x in range(num_points):
                if distribution == 'uniform':
                    point = get_random_point(poly)
                if distribution == 'normal':
                    point = get_random_point(poly, 'normal')
                points_list.append(point)
            points = geometry.MultiPoint(points_list)
            vor_result = ops.voronoi_diagram(points, envelope=poly, edges=False)  # Renamed variable
            for polygon in vor_result.geoms:  # Updated reference
                polygon = polygon.intersection(poly)
                poly_list.append(polygon)

            new_polygons = []
            for current_poly in poly_list:
                if min_iterations >= max_iterations:
                    num_iterations = min_iterations
                else:
                    num_iterations = random.randint(min_iterations, max_iterations)

                if num_iterations > 0:
                    new_polygons.extend(
                        vor_diag(current_poly, num_points, max_iterations=num_iterations - 1,
                                 min_iterations=min_iterations - 1, distribution=distribution))
                else:
                    new_polygons.append(current_poly)

            return new_polygons

        def get_random_point(poly, distribution='uniform'):
            minx, miny, maxx, maxy = poly.bounds
            while True:
                if distribution == 'uniform':
                    p = geometry.Point(random.uniform(minx, maxx), random.uniform(miny, maxy))
                if distribution == 'normal':
                    mu_width = ((maxx - minx) / 2) + minx
                    mu_height = ((maxy - miny) / 2) + miny
                    sigma_width = (maxx - minx) / 4
                    sigma_height = (maxy - miny) / 4
                    p = geometry.Point(random.normalvariate(mu_width, sigma_width),
                                       random.normalvariate(mu_height, sigma_height))
                if poly.contains(p):
                    return p

        center = vsk.width * 0.5, vsk.height * 0.5
        avg_radius, irregularity, spikiness, num_vertices = self.avg_radius, self.irregularity, self.spikiness, self.num_vertices
        rand_poly = generate_polygon(center, avg_radius, irregularity, spikiness, num_vertices)

        if self.poly_spin_smaller:
            times_to_reduce, scale_amount = self.times_to_reduce, self.scale_amount
            rotation_angle = vsk.random(self.rotation_angle_min, self.rotation_angle_max)
            spun_poly = spin_smaller(rand_poly, times_to_reduce, rotation_angle, scale_amount)
            vsk.geometry(spun_poly)

        if self.poly_vor_diag:
            vor_num_points = self.vor_num_initial_points
            vor_poly = vor_diag(rand_poly, vor_num_points, self.vor_min_levels, self.vor_max_levels)
            [vsk.geometry(poly) for poly in vor_poly]

        vsk.vpype(f"penwidth {pen}")

    def finalize(self, vsk: vsketch.Vsketch) -> None:
        vsk.vpype("linemerge --tolerance 0.2mm reloop linesort linesimplify")
        if self.inch_border:
            if vsk.document.page_size[0] > vsk.document.page_size[1]:
                vsk.vpype(
                    f"pagesize -l {vsk.document.page_size[0] + (96 * 2)}x{vsk.document.page_size[1] + (96 * 2)}")
            else:
                vsk.vpype(
                    f"pagesize {vsk.document.page_size[0] + (96 * 2)}x{vsk.document.page_size[1] + (96 * 2)}")
            vsk.vpype(f"translate 96 96")
        if self.half_inch_border:
            if vsk.document.page_size[0] > vsk.document.page_size[1]:
                vsk.vpype(
                    f"pagesize -l {vsk.document.page_size[0] + (96 * 1)}x{vsk.document.page_size[1] + (96 * 1)}")
            else:
                vsk.vpype(
                    f"pagesize {vsk.document.page_size[0] + (96 * 1)}x{vsk.document.page_size[1] + (96 * 1)}")
            vsk.vpype(f"translate 48 48")

if __name__ == "__main__":
    sketch_091523_2222_asteroids.display()
